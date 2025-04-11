use relm4::{
    actions::{AccelsPlus, RelmAction, RelmActionGroup},
    gtk::{PopoverMenuBar, glib::clone, prelude::*},
    prelude::*,
};
use relm4_components::{
    open_dialog::{OpenDialog, OpenDialogMsg, OpenDialogResponse, OpenDialogSettings},
    save_dialog::{SaveDialog, SaveDialogMsg, SaveDialogResponse, SaveDialogSettings},
};
use sourceview5::{
    LanguageManager,
    prelude::{BufferExt, MapExt},
};
use std::{
    fs::{File, exists},
    io::Write,
    path::PathBuf,
};

mod widget_module;
use widget_module::{setup_editor, update_syntax};
mod menu_module;
use menu_module::menu_bar;

struct MainStruct {
    // Non-widgets
    current_file_path: String,
    // Widgets
    buffer: sourceview5::Buffer,
    language_manager: LanguageManager,
    open_dialog: Controller<OpenDialog>,
    save_as_dialog: Controller<SaveDialog>,
    file_label: gtk::Label,
    cursor_position_label: gtk::Label,
    clipboard: gtk::gdk::Clipboard,
}

struct WidgetStruct {}

#[derive(Debug)]
enum Message {
    NewFile,
    OpenRequest,
    OpenResponse(PathBuf),
    SaveAsRequest,
    SaveAsResponse(PathBuf),
    SaveFile,
    ClearEditor,
    CutEditor,
    CopyEditor,
    PasteEditor,
    CursorPostitionChanged,
    Ignore,
}

impl SimpleComponent for MainStruct {
    type Init = String;
    type Input = Message;
    type Output = ();
    type Root = gtk::Window;
    type Widgets = WidgetStruct;

    fn init_root() -> Self::Root {
        gtk::Window::builder().title("Editor").build()
    }

    fn init(
        current_file_path: Self::Init,
        root: Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        // Create menu
        let menu = PopoverMenuBar::from_model(Some(&menu_bar()));

        // Define containers
        let main_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();
        let editor_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .build();
        let status_bar_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .build();
        status_bar_box.set_spacing(10);
        let editor_scroll_window = gtk::ScrolledWindow::builder().build();
        editor_scroll_window.set_policy(gtk4::PolicyType::Automatic, gtk4::PolicyType::Automatic);

        // Define and setup file dialogs
        let load_file_dialog = OpenDialog::builder()
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
        let language_manager = LanguageManager::builder().build();
        let buffer = sourceview5::Buffer::builder().build();
        let buffer_style = sourceview5::StyleSchemeManager::new().scheme("Adwaita-dark");
        buffer.set_style_scheme(buffer_style.as_ref());
        buffer.set_highlight_syntax(true);
        buffer.set_highlight_matching_brackets(true);
        let mini_map = sourceview5::Map::builder().build();
        let editor = setup_editor(&buffer);
        mini_map.set_view(&editor);
        let file_label = gtk::Label::builder().build();
        let cursor_position_label = gtk::Label::builder().build();

        // Add widgets to containers
        editor_scroll_window.set_child(Some(&editor));
        status_bar_box.append(&file_label);
        status_bar_box.append(&cursor_position_label);
        main_box.append(&menu);
        editor_box.append(&editor_scroll_window);
        editor_box.append(&mini_map);
        main_box.append(&editor_box);
        main_box.append(&status_bar_box);

        // Setup the window
        root.set_child(Some(&main_box));
        root.set_default_size(500, 500);

        // Setup events
        buffer.connect_cursor_position_notify(clone!(
            #[strong]
            sender,
            move |_| sender.input(Message::CursorPostitionChanged)
        ));

        // Setup actions
        let program = relm4::main_application();
        program.set_accelerators_for_action::<NewFileAction>(&["<control><shift>n"]);
        program.set_accelerators_for_action::<OpenAction>(&["<control>o"]);
        program.set_accelerators_for_action::<SaveAction>(&["<control>s"]);
        program.set_accelerators_for_action::<SaveAsAction>(&["<control><shift>s"]);
        // File actions
        let new_file_action: RelmAction<NewFileAction> = RelmAction::new_stateless(clone!(
            #[strong]
            sender,
            move |_| sender.input(Message::NewFile)
        ));
        let save_as_action: RelmAction<SaveAsAction> = RelmAction::new_stateless(clone!(
            #[strong]
            sender,
            move |_| sender.input(Message::SaveAsRequest)
        ));
        let save_action: RelmAction<SaveAction> = RelmAction::new_stateless(clone!(
            #[strong]
            sender,
            move |_| sender.input(Message::SaveFile)
        ));
        let open_action: RelmAction<OpenAction> = RelmAction::new_stateless(clone!(
            #[strong]
            sender,
            move |_| sender.input(Message::OpenRequest)
        ));
        // Edit actions
        let cut_action: RelmAction<CutAction> = RelmAction::new_stateless(clone!(
            #[strong]
            sender,
            move |_| sender.input(Message::CutEditor)
        ));
        let copy_action: RelmAction<CopyAction> = RelmAction::new_stateless(clone!(
            #[strong]
            sender,
            move |_| sender.input(Message::CopyEditor)
        ));
        let paste_action: RelmAction<PasteAction> = RelmAction::new_stateless(clone!(
            #[strong]
            sender,
            move |_| sender.input(Message::PasteEditor)
        ));
        let clear_action: RelmAction<ClearAction> = RelmAction::new_stateless(clone!(
            #[strong]
            sender,
            move |_| sender.input(Message::ClearEditor)
        ));
        // Add actions to group
        let mut action_group = RelmActionGroup::<WindowActionGroup>::new();
        action_group.add_action(new_file_action);
        action_group.add_action(save_as_action);
        action_group.add_action(save_action);
        action_group.add_action(open_action);
        action_group.add_action(cut_action);
        action_group.add_action(copy_action);
        action_group.add_action(paste_action);
        action_group.add_action(clear_action);
        action_group.register_for_widget(&root);

        // Set misc variables
        let display = gtk::gdk::Display::default().unwrap();
        let clipboard = DisplayExt::clipboard(&display);

        let model = MainStruct {
            current_file_path,
            buffer,
            language_manager,
            open_dialog: load_file_dialog,
            save_as_dialog,
            file_label,
            cursor_position_label,
            clipboard,
        };
        let widgets = WidgetStruct {};
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: relm4::ComponentSender<Self>) {
        match message {
            Message::NewFile => {
                self.buffer.set_text("");
                self.current_file_path = "".to_string();
            }
            Message::OpenRequest => self.open_dialog.emit(OpenDialogMsg::Open),
            Message::OpenResponse(path) => match std::fs::read_to_string(&path) {
                Ok(f) => {
                    self.buffer.set_text(&f);
                    self.current_file_path = Some(path.to_str().unwrap().to_string()).unwrap();
                    match update_syntax(&self.language_manager, &self.current_file_path) {
                        Some(language) => {
                            self.buffer.set_language(Some(&language));
                        }
                        None => {
                            //
                        }
                    }
                }
                Err(_) => panic!("Failed to read file to string!"),
            },
            Message::SaveAsRequest => self
                .save_as_dialog
                .emit(SaveDialogMsg::SaveAs("".to_string())),
            Message::SaveAsResponse(path) => match std::fs::write(
                &path,
                self.buffer
                    .text(&self.buffer.start_iter(), &self.buffer.end_iter(), false),
            ) {
                Ok(_) => {
                    // Pass
                }
                Err(_) => {
                    // Pass
                }
            },
            Message::SaveFile => {
                match exists(&self.current_file_path) {
                    Ok(_) => match File::create(&self.current_file_path) {
                        Ok(f) => {
                            let mut file = f;
                            file.write_all(
                                self.buffer
                                    .text(&self.buffer.start_iter(), &self.buffer.end_iter(), false)
                                    .as_bytes(),
                            )
                            .unwrap();
                        }
                        Err(_) => {
                            //
                        }
                    },
                    Err(_) => {
                        // Pass
                    }
                }
            }
            Message::CutEditor => {
                self.buffer.cut_clipboard(&self.clipboard, true);
            }
            Message::CopyEditor => {
                self.buffer.copy_clipboard(&self.clipboard);
            }
            Message::PasteEditor => {
                self.buffer.paste_clipboard(&self.clipboard, None, true);
            }
            Message::ClearEditor => {
                self.buffer.set_text("");
            }
            Message::CursorPostitionChanged => {
                // Pass
            }
            Message::Ignore => {
                // Pass
            }
        }
    }
    fn update_view(&self, _widgets: &mut Self::Widgets, _sender: ComponentSender<Self>) {
        self.file_label.set_label(&self.current_file_path);
        self.cursor_position_label
            .set_label(&self.buffer.cursor_position().to_string().as_str());
    }
}

relm4::new_action_group!(WindowActionGroup, "win");
relm4::new_stateless_action!(NewFileAction, WindowActionGroup, "new_file");
relm4::new_stateless_action!(SaveAsAction, WindowActionGroup, "save_as");
relm4::new_stateless_action!(SaveAction, WindowActionGroup, "save");
relm4::new_stateless_action!(OpenAction, WindowActionGroup, "open");
relm4::new_stateless_action!(CutAction, WindowActionGroup, "cut");
relm4::new_stateless_action!(CopyAction, WindowActionGroup, "copy");
relm4::new_stateless_action!(PasteAction, WindowActionGroup, "paste");
relm4::new_stateless_action!(ClearAction, WindowActionGroup, "clear");

fn main() {
    let program = RelmApp::new("editor.cyncrovee");
    program.run::<MainStruct>("".to_string());
}
