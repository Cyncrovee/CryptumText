use relm4::{
    RelmRemoveAllExt,
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
    fs::{File, exists, read_dir},
    io::Write,
    path::{Path, PathBuf},
};

mod widget_module;
use widget_module::{setup_editor, update_file_type};

mod menu_module;
use menu_module::menu_bar;

mod fs_module;
use fs_module::load_file;

mod program_model;
use program_model::{MainStruct, Message, WidgetStruct};

impl SimpleComponent for MainStruct {
    type Init = String;
    type Input = Message;
    type Output = ();
    type Root = gtk::Window;
    type Widgets = WidgetStruct;

    fn init_root() -> Self::Root {
        gtk::Window::builder().title("Cryptum Text").build()
    }

    fn init(
        current_file_path: Self::Init,
        root: Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        // Enable libadwaita (commented out for now)
        //let program = gtk::Application::default();
        //program.connect_startup(|_| libadwaita::init().unwrap());

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
        let editor_scroll_window = gtk::ScrolledWindow::builder().build();
        editor_scroll_window.set_policy(gtk4::PolicyType::Automatic, gtk4::PolicyType::Automatic);

        // Define and setup file dialogs
        let mut load_folder_dialog_settings = OpenDialogSettings::default();
        load_folder_dialog_settings.folder_mode = true;
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
        let folder_label = gtk::Label::builder().build();
        let file_list = gtk::ListBox::builder().build();
        let language_manager = LanguageManager::builder().build();
        let buffer = sourceview5::Buffer::builder().build();
        let buffer_style = sourceview5::StyleSchemeManager::new().scheme("Adwaita-dark");
        buffer.set_style_scheme(buffer_style.as_ref());
        buffer.set_highlight_syntax(true);
        buffer.set_highlight_matching_brackets(true);
        let editor = setup_editor(&buffer);
        let mini_map = sourceview5::Map::builder().build();
        mini_map.set_view(&editor);
        let file_type_label = gtk::Label::builder().build();
        let file_label = gtk::Label::builder().build();
        let cursor_position_label = gtk::Label::builder().build();

        // Add widgets to containers
        editor_scroll_window.set_child(Some(&editor));
        status_bar_box.append(&file_label);
        status_bar_box.append(&gtk::Label::builder().label(" | ").build());
        status_bar_box.append(&file_type_label);
        status_bar_box.append(&gtk::Label::builder().label(" | ").build());
        status_bar_box.append(&cursor_position_label);
        main_box.append(&menu);
        main_box.append(&folder_label);
        editor_box.append(&file_list);
        editor_box.append(&editor_scroll_window);
        editor_box.append(&mini_map);
        main_box.append(&editor_box);
        main_box.append(&status_bar_box);

        // Setup the window
        root.set_child(Some(&main_box));
        root.set_default_size(1000, 1000);

        // Setup events
        file_list.connect_selected_rows_changed(clone!(
            #[strong]
            sender,
            move |_| sender.input(Message::LoadFileFromList)
        ));
        buffer.connect_cursor_position_notify(clone!(
            #[strong]
            sender,
            move |_| sender.input(Message::CursorPostitionChanged)
        ));

        // Setup actions
        let program = relm4::main_application();
        program.set_accelerators_for_action::<NewFileAction>(&["<control><shift>n"]);
        program.set_accelerators_for_action::<OpenAction>(&["<control>o"]);
        program.set_accelerators_for_action::<OpenFolderAction>(&["<control><shift>o"]);
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
        let open_folder_action: RelmAction<OpenFolderAction> = RelmAction::new_stateless(clone!(
            #[strong]
            sender,
            move |_| sender.input(Message::FolderRequest)
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
        action_group.add_action(open_folder_action);
        action_group.add_action(cut_action);
        action_group.add_action(copy_action);
        action_group.add_action(paste_action);
        action_group.add_action(clear_action);
        action_group.register_for_widget(&root);

        // Set misc variables
        let display = gtk::gdk::Display::default().unwrap();
        let clipboard = DisplayExt::clipboard(&display);
        let current_folder_path = "".into();

        let model = MainStruct {
            // Non-Widgets
            current_file_path,
            current_folder_path,
            clipboard,
            // Widgets
            file_list,
            buffer,
            language_manager,
            open_dialog,
            folder_dialog,
            save_as_dialog,
            file_label,
            folder_label,
            file_type_label,
            cursor_position_label,
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
            Message::LoadFileFromList => {
                let mut file_list_pathbuf = PathBuf::from(&self.current_folder_path);
                let file_list_name = &self
                    .file_list
                    .selected_row()
                    .unwrap()
                    .child()
                    .unwrap()
                    .widget_name();
                let file_list_path = Path::new(file_list_name);
                file_list_pathbuf.push(file_list_path);
                match PathBuf::from(file_list_name).is_file() {
                    true => {
                        self.current_file_path =
                            file_list_pathbuf.into_os_string().into_string().unwrap();
                        load_file(self);
                    }
                    false => {
                        //
                    }
                }
            }
            Message::FolderRequest => self.folder_dialog.emit(OpenDialogMsg::Open),
            Message::FolderResponse(path) => {
                self.current_folder_path = path.clone().into_os_string().into_string().unwrap();
                match read_dir(&path.clone()) {
                    Ok(dir) => {
                        self.file_list.remove_all();
                        for files in dir {
                            let label = gtk::Label::builder().build();
                            label.set_widget_name(
                                files
                                    .as_ref()
                                    .unwrap()
                                    .file_name()
                                    .as_os_str()
                                    .to_str()
                                    .unwrap(),
                            );
                            label
                                .set_text(files.unwrap().file_name().as_os_str().to_str().unwrap());
                            self.file_list.append(&label);
                        }
                    }
                    Err(_) => {
                        //
                    }
                }
            }
            Message::OpenRequest => self.open_dialog.emit(OpenDialogMsg::Open),
            Message::OpenResponse(path) => {
                self.current_file_path = path.into_os_string().into_string().unwrap();
                load_file(self);
            }
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
        self.folder_label.set_label(&self.current_folder_path);
        match update_file_type(&self.current_file_path) {
            Some(file_type) => {
                self.file_type_label.set_label(&file_type);
            }
            None => {
                self.file_type_label.set_label("");
            }
        }
        self.cursor_position_label
            .set_label(&self.buffer.cursor_position().to_string().as_str());
    }
}

relm4::new_action_group!(WindowActionGroup, "win");
relm4::new_stateless_action!(NewFileAction, WindowActionGroup, "new_file");
relm4::new_stateless_action!(SaveAsAction, WindowActionGroup, "save_as");
relm4::new_stateless_action!(SaveAction, WindowActionGroup, "save");
relm4::new_stateless_action!(OpenAction, WindowActionGroup, "open");
relm4::new_stateless_action!(OpenFolderAction, WindowActionGroup, "open_folder");
relm4::new_stateless_action!(CutAction, WindowActionGroup, "cut");
relm4::new_stateless_action!(CopyAction, WindowActionGroup, "copy");
relm4::new_stateless_action!(PasteAction, WindowActionGroup, "paste");
relm4::new_stateless_action!(ClearAction, WindowActionGroup, "clear");

fn main() {
    let program = RelmApp::new("editor.cyncrovee");
    program.run::<MainStruct>("".to_string());
}
