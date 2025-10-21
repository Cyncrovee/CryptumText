use std::path::PathBuf;

use gtk4::{MenuButton, ScrolledWindow};
use libadwaita::{HeaderBar, ToastOverlay, WindowTitle, prelude::*};
use relm4::{
    actions::{AccelsPlus, RelmAction, RelmActionGroup},
    gtk::glib::clone,
    prelude::*,
};
use relm4_components::{
    open_dialog::{OpenDialog, OpenDialogResponse, OpenDialogSettings},
    save_dialog::{SaveDialog, SaveDialogResponse, SaveDialogSettings},
};
use sourceview5::LanguageManager;

mod app;
use app::{
    model::{Message, State, WidgetStruct},
    update::handle_messages,
    view::handle_view,
};

mod util;
use util::{menu::menu_bar, widget::setup_editor};
mod fs;

impl SimpleComponent for State {
    type Init = PathBuf;
    type Input = Message;
    type Output = ();
    type Root = libadwaita::ApplicationWindow;
    type Widgets = WidgetStruct;

    fn init_root() -> Self::Root {
        libadwaita::ApplicationWindow::builder().build()
    }

    fn init(
        current_file_path: Self::Init,
        root: Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        // Enable libadwaita
        let program = libadwaita::Application::builder().build();
        program
            .style_manager()
            .set_color_scheme(libadwaita::ColorScheme::Default);
        program.connect_startup(|_| libadwaita::init().unwrap());

        // Define and setup dialogs
        let load_folder_dialog_settings = OpenDialogSettings {
            folder_mode: true,
            ..Default::default()
        };
        let folder_dialog = OpenDialog::builder()
            .transient_for_native(&root)
            .launch(load_folder_dialog_settings)
            .forward(sender.input_sender(), |response| match response {
                OpenDialogResponse::Accept(path) => Message::FolderResponse(path),
                OpenDialogResponse::Cancel => Message::Ignore,
            });
        let open_dialog = OpenDialog::builder()
            .transient_for_native(&root)
            .launch(OpenDialogSettings::default())
            .forward(sender.input_sender(), |response| match response {
                OpenDialogResponse::Accept(path) => Message::OpenResponse(path),
                OpenDialogResponse::Cancel => Message::Ignore,
            });
        let save_as_dialog = SaveDialog::builder()
            .transient_for_native(&root)
            .launch(SaveDialogSettings::default())
            .forward(sender.input_sender(), |response| match response {
                SaveDialogResponse::Accept(path) => Message::SaveAsResponse(path),
                SaveDialogResponse::Cancel => Message::Ignore,
            });

        // Define and edit widgets
        let title = WindowTitle::new("Cryptum Text", "");
        let hamburger = MenuButton::builder()
            .icon_name("open-menu-symbolic")
            .menu_model(&menu_bar())
            .build();
        let header = HeaderBar::builder().title_widget(&title).build();
        header.pack_end(&hamburger);
        let file_view = gtk::ListView::builder()
            .css_classes(vec!["navigation-sidebar"])
            .vexpand(true)
            .orientation(gtk4::Orientation::Vertical)
            .height_request(500)
            .build();
        let file_view_scroll = ScrolledWindow::builder()
            .hscrollbar_policy(gtk4::PolicyType::Never)
            .child(&file_view)
            .build();
        let language_manager = LanguageManager::builder().build();
        let buffer_style = sourceview5::StyleSchemeManager::new().scheme("Adwaita-dark");
        let buffer = sourceview5::Buffer::builder()
            .style_scheme(buffer_style.as_ref().unwrap())
            .highlight_matching_brackets(true)
            .build();
        let editor = setup_editor(&buffer);
        let mini_map = sourceview5::Map::builder()
            .width_request(120)
            .overflow(gtk4::Overflow::Visible)
            .view(&editor)
            .build();
        let file_type_label = gtk::Label::builder().halign(gtk4::Align::Start).build();
        let cursor_position_label = gtk::Label::builder().halign(gtk4::Align::End).build();
        let toast_overlay = ToastOverlay::new();

        // Define containers
        let main_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();
        let side_bar_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();
        let file_list_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();
        let editor_box_vertical = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();
        let editor_box_horizontal = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .build();
        let status_bar_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .homogeneous(true)
            .build();
        let editor_scroll_window = gtk::ScrolledWindow::builder()
            .hscrollbar_policy(gtk4::PolicyType::Automatic)
            .vscrollbar_policy(gtk4::PolicyType::Automatic)
            .build();

        // Add widgets to containers
        editor_scroll_window.set_child(Some(&editor));
        status_bar_box.append(&file_type_label);
        status_bar_box.append(&cursor_position_label);
        file_list_box.append(&file_view_scroll);
        side_bar_box.append(&file_list_box);
        toast_overlay.set_child(Some(&editor_box_vertical));
        editor_box_horizontal.append(&side_bar_box);
        editor_box_horizontal.append(&editor_scroll_window);
        editor_box_horizontal.append(&mini_map);
        editor_box_vertical.append(&editor_box_horizontal);
        editor_box_vertical.append(&status_bar_box);
        main_box.append(&header);
        main_box.append(&toast_overlay);

        // Setup the window
        root.set_content(Some(&main_box));
        root.set_default_size(1000, 1000);

        // Set misc variables
        let current_folder_path = "".into();
        let view_hidden = true;

        // Apply user settings
        root.connect_show(clone!(
            #[strong]
            sender,
            move |_| sender.input(Message::LoadSettings)
        ));

        // Setup events/gestures
        buffer.connect_cursor_position_notify(clone!(
            #[strong]
            sender,
            move |_| sender.input(Message::CursorPositionChanged)
        ));

        // Setup actions
        let program = relm4::main_application();
        // File accelerators
        program.set_accelerators_for_action::<NewFileAction>(&["<control><shift>n"]);
        program.set_accelerators_for_action::<OpenAction>(&["<control>o"]);
        program.set_accelerators_for_action::<OpenFolderAction>(&["<control><shift>o"]);
        program.set_accelerators_for_action::<SaveAction>(&["<control>s"]);
        program.set_accelerators_for_action::<SaveAsAction>(&["<control><shift>s"]);
        // View accelerators
        program.set_accelerators_for_action::<ToggleFileListAction>(&["<control><alt>f"]);
        program.set_accelerators_for_action::<ToggleHiddenFilesAction>(&["<control>h"]);
        program.set_accelerators_for_action::<ToggleMiniMapAction>(&["<control><alt>m"]);
        program.set_accelerators_for_action::<ToggleFullscreenAction>(&["F11"]);
        // About accelerators
        program.set_accelerators_for_action::<ShowPreferencesAction>(&["<control>comma"]);
        program.set_accelerators_for_action::<ShowKeyboardShortcutsAction>(&["<control>question"]);

        // Create action groups and add actions to them
        let mut file_action_group = RelmActionGroup::<FileActionGroup>::new();
        let mut edit_action_group = RelmActionGroup::<EditActionGroup>::new();
        let mut view_action_group = RelmActionGroup::<ViewActionGroup>::new();
        let mut about_action_group = RelmActionGroup::<AboutActionGroup>::new();
        // File actions
        file_action_group.add_action(RelmAction::<NewFileAction>::new_stateless(clone!(
            #[strong]
            sender,
            move |_| sender.input(Message::NewFile)
        )));
        file_action_group.add_action(RelmAction::<SaveAsAction>::new_stateless(clone!(
            #[strong]
            sender,
            move |_| sender.input(Message::SaveAsRequest)
        )));
        file_action_group.add_action(RelmAction::<SaveAction>::new_stateless(clone!(
            #[strong]
            sender,
            move |_| sender.input(Message::SaveFile)
        )));
        file_action_group.add_action(RelmAction::<OpenAction>::new_stateless(clone!(
            #[strong]
            sender,
            move |_| sender.input(Message::OpenRequest)
        )));
        file_action_group.add_action(RelmAction::<OpenFolderAction>::new_stateless(clone!(
            #[strong]
            sender,
            move |_| sender.input(Message::FolderRequest)
        )));
        // Edit actions
        edit_action_group.add_action(RelmAction::<ClearAction>::new_stateless(clone!(
            #[strong]
            sender,
            move |_| sender.input(Message::ClearEditor)
        )));
        // View actions
        view_action_group.add_action(RelmAction::<ToggleFileListAction>::new_stateless(clone!(
            #[strong]
            sender,
            move |_| sender.input(Message::ToggleFileList)
        )));
        view_action_group.add_action(RelmAction::<ToggleHiddenFilesAction>::new_stateless(
            clone!(
                #[strong]
                sender,
                move |_| sender.input(Message::ToggleHiddenFiles)
            ),
        ));
        view_action_group.add_action(RelmAction::<ToggleMiniMapAction>::new_stateless(clone!(
            #[strong]
            sender,
            move |_| sender.input(Message::ToggleMiniMap)
        )));
        view_action_group.add_action(RelmAction::<ToggleBufferStyleAction>::new_stateless(
            clone!(
                #[strong]
                sender,
                move |_| sender.input(Message::ToggleBufferStyleScheme)
            ),
        ));
        view_action_group.add_action(RelmAction::<ToggleFullscreenAction>::new_stateless(clone!(
            #[strong]
            sender,
            move |_| sender.input(Message::ToggleFullscreen)
        )));
        // About actions
        about_action_group.add_action(RelmAction::<ShowKeyboardShortcutsAction>::new_stateless(
            clone!(
                #[strong]
                sender,
                move |_| sender.input(Message::ShowKeyboardShortcuts)
            ),
        ));
        about_action_group.add_action(RelmAction::<ShowPreferencesAction>::new_stateless(clone!(
            #[strong]
            sender,
            move |_| sender.input(Message::ShowPreferences)
        )));
        about_action_group.add_action(RelmAction::<ShowAboutAction>::new_stateless(clone!(
            #[strong]
            sender,
            move |_| sender.input(Message::ShowAbout)
        )));

        // Register action groups
        file_action_group.register_for_widget(&root);
        edit_action_group.register_for_widget(&root);
        view_action_group.register_for_widget(&root);
        about_action_group.register_for_widget(&root);

        let model = State {
            // Containers
            root,
            side_bar_box,
            // Widgets
            file_view,
            editor,
            buffer,
            language_manager,
            open_dialog,
            folder_dialog,
            save_as_dialog,
            title,
            file_type_label,
            cursor_position_label,
            mini_map,
            toast_overlay,
            // Misc
            current_file_path,
            current_folder_path,
            buffer_style,
            view_hidden,
        };
        let widgets = WidgetStruct {};
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: relm4::ComponentSender<Self>) {
        handle_messages(self, message, sender);
    }
    fn update_view(&self, _widgets: &mut Self::Widgets, _sender: ComponentSender<Self>) {
        handle_view(self, _widgets, _sender);
    }
}

// Groups
relm4::new_action_group!(FileActionGroup, "file");
relm4::new_action_group!(EditActionGroup, "edit");
relm4::new_action_group!(ViewActionGroup, "view");
relm4::new_action_group!(AboutActionGroup, "about");
// File
relm4::new_stateless_action!(NewFileAction, FileActionGroup, "new_file");
relm4::new_stateless_action!(SaveAsAction, FileActionGroup, "save_as");
relm4::new_stateless_action!(SaveAction, FileActionGroup, "save");
relm4::new_stateless_action!(OpenAction, FileActionGroup, "open");
relm4::new_stateless_action!(OpenFolderAction, FileActionGroup, "open_folder");
// Edit
relm4::new_stateless_action!(ClearAction, EditActionGroup, "clear");
// View
relm4::new_stateless_action!(ToggleFileListAction, ViewActionGroup, "toggle_file_list");
relm4::new_stateless_action!(
    ToggleHiddenFilesAction,
    ViewActionGroup,
    "toggle_hidden_files"
);
relm4::new_stateless_action!(ToggleMiniMapAction, ViewActionGroup, "toggle_mini_map");
relm4::new_stateless_action!(
    ToggleBufferStyleAction,
    ViewActionGroup,
    "toggle_buffer_style_scheme"
);
relm4::new_stateless_action!(ToggleFullscreenAction, ViewActionGroup, "toggle_fullscreen");
// About
relm4::new_stateless_action!(
    ShowKeyboardShortcutsAction,
    AboutActionGroup,
    "show_keyboard_shortcuts"
);
relm4::new_stateless_action!(ShowPreferencesAction, AboutActionGroup, "show_preferences");
relm4::new_stateless_action!(ShowAboutAction, AboutActionGroup, "show_about");

fn main() {
    let program = RelmApp::new("io.github.Cyncrovee.CryptumText");
    program.run::<State>(PathBuf::new());
}
