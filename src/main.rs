use gtk4::{AboutDialog, MenuButton};
use libadwaita::{HeaderBar, WindowTitle, prelude::*};
use relm4::{
    actions::{AccelsPlus, RelmAction, RelmActionGroup},
    gtk::glib::clone,
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
    path::{Path, PathBuf},
};

mod widget_module;
use widget_module::{setup_editor, update_file_type};

mod menu_module;
use menu_module::{extras_menu_bar, menu_bar};

mod fs_module;
use fs_module::{load_file, load_folder, load_settings, save_settings};

mod program_model;
use program_model::{MainStruct, Message, WidgetStruct};

impl SimpleComponent for MainStruct {
    type Init = String;
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

        let hamburger = MenuButton::builder()
            .icon_name("open-menu-symbolic")
            .menu_model(&menu_bar())
            .build();
        let extras = MenuButton::builder()
            .icon_name("help-about-symbolic")
            .menu_model(&extras_menu_bar())
            .build();

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
        let title = WindowTitle::new("Cryptum Text", "");
        let header = HeaderBar::builder().title_widget(&title).build();
        header.pack_start(&hamburger);
        header.pack_end(&extras);
        let file_list = gtk::ListBox::builder()
            .css_classes(vec!["navigation-sidebar"])
            .build();
        let language_manager = LanguageManager::builder().build();
        let buffer = sourceview5::Buffer::builder().build();
        let buffer_style = sourceview5::StyleSchemeManager::new().scheme("Adwaita-dark");
        let current_style = "Dark".to_string();
        buffer.set_style_scheme(buffer_style.as_ref());
        buffer.set_highlight_matching_brackets(true);
        let editor = setup_editor(&buffer);
        let mini_map = sourceview5::Map::builder().build();
        mini_map.set_width_request(120);
        mini_map.set_view(&editor);
        let file_type_label = gtk::Label::builder().build();
        let cursor_position_label = gtk::Label::builder().build();

        // Add widgets to containers
        editor_scroll_window.set_child(Some(&editor));
        status_bar_box.append(&gtk::Label::builder().label("   ").build());
        status_bar_box.append(&file_type_label);
        status_bar_box.append(&gtk::Label::builder().label(" | ").build());
        status_bar_box.append(&cursor_position_label);
        main_box.append(&header);
        editor_box.append(&file_list);
        editor_box.append(&editor_scroll_window);
        editor_box.append(&mini_map);
        main_box.append(&editor_box);
        main_box.append(&status_bar_box);

        // Setup the window
        root.set_content(Some(&main_box));
        root.set_default_size(1000, 1000);

        // Apply user settings
        load_settings(&file_list, &mini_map);

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
        // File accelerators
        program.set_accelerators_for_action::<NewFileAction>(&["<control><shift>n"]);
        program.set_accelerators_for_action::<OpenAction>(&["<control>o"]);
        program.set_accelerators_for_action::<OpenFolderAction>(&["<control><shift>o"]);
        program.set_accelerators_for_action::<SaveAction>(&["<control>s"]);
        program.set_accelerators_for_action::<SaveAsAction>(&["<control><shift>s"]);
        // Edit accelerators
        program.set_accelerators_for_action::<UndoAction>(&["<control>z"]);
        program.set_accelerators_for_action::<RedoAction>(&["<control>y"]);
        program.set_accelerators_for_action::<CutAction>(&["<control>x"]);
        program.set_accelerators_for_action::<CopyAction>(&["<control>c"]);
        program.set_accelerators_for_action::<PasteAction>(&["<control>v"]);
        // View accelerators
        program.set_accelerators_for_action::<ToggleFileListAction>(&["<control><alt>f"]);
        program.set_accelerators_for_action::<ToggleMiniMapAction>(&["<control><alt>m"]);
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
        let undo_action: RelmAction<UndoAction> = RelmAction::new_stateless(clone!(
            #[strong]
            sender,
            move |_| sender.input(Message::Undo)
        ));
        let redo_action: RelmAction<RedoAction> = RelmAction::new_stateless(clone!(
            #[strong]
            sender,
            move |_| sender.input(Message::Redo)
        ));
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
        // View actions
        let toggle_file_list_action: RelmAction<ToggleFileListAction> =
            RelmAction::new_stateless(clone!(
                #[strong]
                sender,
                move |_| sender.input(Message::ToggleFileList)
            ));
        let toggle_mini_map_action: RelmAction<ToggleMiniMapAction> =
            RelmAction::new_stateless(clone!(
                #[strong]
                sender,
                move |_| sender.input(Message::ToggleMiniMap)
            ));
        let toggle_buffer_style_scheme_action: RelmAction<ToggleBufferStyleAction> =
            RelmAction::new_stateless(clone!(
                #[strong]
                sender,
                move |_| sender.input(Message::ToggleBufferStyleScheme)
            ));
        // About actions
        let show_about_action: RelmAction<ShowAboutAction> = RelmAction::new_stateless(clone!(
            #[strong]
            sender,
            move |_| sender.input(Message::ShowAbout)
        ));
        // Add actions to group
        let mut file_action_group = RelmActionGroup::<FileActionGroup>::new();
        let mut edit_action_group = RelmActionGroup::<EditActionGroup>::new();
        let mut view_action_group = RelmActionGroup::<ViewActionGroup>::new();
        let mut about_action_group = RelmActionGroup::<AboutActionGroup>::new();
        file_action_group.add_action(new_file_action);
        file_action_group.add_action(save_as_action);
        file_action_group.add_action(save_action);
        file_action_group.add_action(open_action);
        file_action_group.add_action(open_folder_action);
        edit_action_group.add_action(undo_action);
        edit_action_group.add_action(redo_action);
        edit_action_group.add_action(cut_action);
        edit_action_group.add_action(copy_action);
        edit_action_group.add_action(paste_action);
        edit_action_group.add_action(clear_action);
        view_action_group.add_action(toggle_file_list_action);
        view_action_group.add_action(toggle_mini_map_action);
        view_action_group.add_action(toggle_buffer_style_scheme_action);
        about_action_group.add_action(show_about_action);
        // Register action groups
        file_action_group.register_for_widget(&root);
        edit_action_group.register_for_widget(&root);
        view_action_group.register_for_widget(&root);
        about_action_group.register_for_widget(&root);

        // Set misc variables
        let display = gtk::gdk::Display::default().unwrap();
        let clipboard = DisplayExt::clipboard(&display);
        let current_folder_path = "".into();

        let model = MainStruct {
            // Non-Widgets
            current_file_path,
            current_folder_path,
            clipboard,
            buffer_style,
            current_style,
            // Widgets
            file_list,
            buffer,
            language_manager,
            open_dialog,
            folder_dialog,
            save_as_dialog,
            title,
            file_type_label,
            cursor_position_label,
            mini_map,
        };
        let widgets = WidgetStruct {};
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: relm4::ComponentSender<Self>) {
        match message {
            // File
            Message::NewFile => {
                self.buffer.set_text("");
                self.current_file_path = "".to_string();
            }
            Message::LoadFileFromList => match self.file_list.selected_row() {
                Some(_) => {
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
                    match PathBuf::from(file_list_name).is_dir() {
                        true => {
                            self.current_folder_path = file_list_name.clone().to_string();
                            let path = file_list_name.clone().to_string();
                            load_folder(self, &path);
                        }
                        false => match PathBuf::from(file_list_name).is_file() {
                            true => {
                                self.current_file_path =
                                    file_list_pathbuf.into_os_string().into_string().unwrap();
                                load_file(self);
                            }
                            false => {
                                let mut owned_folder = PathBuf::from(&self.current_folder_path);
                                owned_folder.push(file_list_name);
                                match owned_folder.is_file() {
                                    true => {
                                        self.current_file_path = file_list_pathbuf
                                            .into_os_string()
                                            .into_string()
                                            .unwrap();
                                        load_file(self);
                                    }
                                    false => {
                                        println!("Selected row not a file!");
                                    }
                                }
                            }
                        },
                    }
                }
                None => {
                    println!("No row selected!");
                }
            },
            Message::FolderRequest => self.folder_dialog.emit(OpenDialogMsg::Open),
            Message::FolderResponse(path) => {
                self.current_folder_path = path.clone().into_os_string().into_string().unwrap();
                load_folder(self, &path.into_os_string().into_string().unwrap());
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
            // Edit
            Message::Undo => {
                self.buffer.undo();
            }
            Message::Redo => {
                self.buffer.redo();
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
                self.buffer.undo();
            }
            // View
            Message::ToggleFileList => {
                self.file_list.set_visible(!self.file_list.is_visible());
                save_settings(self);
            }
            Message::ToggleMiniMap => {
                self.mini_map.set_visible(!self.mini_map.is_visible());
                save_settings(self);
            }
            Message::ToggleBufferStyleScheme => {
                match self.current_style.as_str() {
                    "Dark" => {
                        self.buffer_style =
                            sourceview5::StyleSchemeManager::new().scheme("Adwaita");
                        self.buffer.set_style_scheme(self.buffer_style.as_ref());
                        self.current_style = "Light".to_string();
                    }
                    "Light" => {
                        self.buffer_style =
                            sourceview5::StyleSchemeManager::new().scheme("Adwaita-dark");
                        self.buffer.set_style_scheme(self.buffer_style.as_ref());
                        self.current_style = "Dark".to_string();
                    }
                    _ => {
                        // Pass
                    }
                }
            }
            // About
            Message::ShowAbout => {
                AboutDialog::builder()
                    .program_name("Cryptum Text")
                    .version("Dev Version")
                    .copyright("Ella Hart (Cyncrovee)")
                    .license_type(gtk4::License::Gpl30Only)
                    .build()
                    .show();
            }
            // Other
            Message::CursorPostitionChanged => {
                // Pass
            }
            Message::Ignore => {
                // Pass
            }
        }
    }
    fn update_view(&self, _widgets: &mut Self::Widgets, _sender: ComponentSender<Self>) {
        self.title.set_subtitle(&self.current_file_path);
        match update_file_type(&self.current_file_path) {
            Some(file_type) => {
                self.file_type_label.set_label(&file_type);
            }
            None => {
                self.file_type_label.set_label("");
            }
        }
        let cursor_iter = &self.buffer.iter_at_offset(self.buffer.cursor_position());
        let cursor_line = cursor_iter.line();
        let cursor_row = cursor_iter.line_offset();
        let mut cursor_position = cursor_line.to_string().to_owned();
        cursor_position.push(':');
        cursor_position.push_str(cursor_row.to_string().as_str());
        self.cursor_position_label.set_label(&cursor_position);
    }
}

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
relm4::new_stateless_action!(UndoAction, EditActionGroup, "undo");
relm4::new_stateless_action!(RedoAction, EditActionGroup, "redo");
relm4::new_stateless_action!(CutAction, EditActionGroup, "cut");
relm4::new_stateless_action!(CopyAction, EditActionGroup, "copy");
relm4::new_stateless_action!(PasteAction, EditActionGroup, "paste");
relm4::new_stateless_action!(ClearAction, EditActionGroup, "clear");
// View
relm4::new_stateless_action!(ToggleFileListAction, ViewActionGroup, "toggle_file_list");
relm4::new_stateless_action!(ToggleMiniMapAction, ViewActionGroup, "toggle_mini_map");
relm4::new_stateless_action!(
    ToggleBufferStyleAction,
    ViewActionGroup,
    "toggle_buffer_style_scheme"
);
// About
relm4::new_stateless_action!(ShowAboutAction, AboutActionGroup, "show_about");

fn main() {
    let program = RelmApp::new("editor.cyncrovee");
    program.run::<MainStruct>("".to_string());
}
