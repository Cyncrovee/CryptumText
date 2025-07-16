use std::{
    fs::{self, File, exists},
    io::Write,
    path::{Path, PathBuf},
};

use gtk4::{gdk::Rectangle, prelude::*};
use relm4::ComponentController;
use relm4_components::{open_dialog::OpenDialogMsg, save_dialog::SaveDialogMsg};
use sourceview5::prelude::{BufferExt, ViewExt};

use crate::{
    app::model::{MainStruct, Message},
    fs::{load_file, load_folder, load_settings, save_settings},
    util::menu::{file_list_context_menu_model, file_list_context_menu_model_item},
};

pub(crate) fn handle_messages(
    main_struct: &mut MainStruct,
    message: Message,
    sender: relm4::ComponentSender<MainStruct>,
) {
    match message {
        // File
        Message::NewFile => {
            main_struct.buffer.set_text("");
            main_struct.current_file_path = "".to_string();
            main_struct.file_list.unselect_all();
        }
        Message::LoadFileFromList => {
            if let Some(row) = main_struct.file_list.selected_row() {
                let mut file_list_pathbuf = PathBuf::from(&main_struct.current_folder_path);
                let file_list_name = row.child().unwrap().widget_name();
                let file_list_path = Path::new(file_list_name.as_str());
                file_list_pathbuf.push(file_list_path);
                match PathBuf::from(&file_list_pathbuf).is_dir() {
                    true => {
                        main_struct.current_folder_path =
                            file_list_pathbuf.into_os_string().into_string().unwrap();
                        let path = main_struct.current_folder_path.clone();
                        load_folder(main_struct, &path);
                    }
                    false => {
                        main_struct.current_file_path =
                            file_list_pathbuf.into_os_string().into_string().unwrap();
                        load_file(main_struct);
                    }
                }
                main_struct.file_list.unselect_all();
            }
        }
        Message::FolderRequest => main_struct.folder_dialog.emit(OpenDialogMsg::Open),
        Message::FolderResponse(path) => {
            main_struct.current_folder_path = path.clone().into_os_string().into_string().unwrap();
            load_folder(main_struct, &path.into_os_string().into_string().unwrap());
        }
        Message::OpenRequest => main_struct.open_dialog.emit(OpenDialogMsg::Open),
        Message::OpenResponse(path) => {
            main_struct.current_file_path = path.into_os_string().into_string().unwrap();
            load_file(main_struct);
        }
        Message::SaveAsRequest => main_struct
            .save_as_dialog
            .emit(SaveDialogMsg::SaveAs("".to_string())),
        Message::SaveAsResponse(path) => match std::fs::write(
            &path,
            main_struct.buffer.text(
                &main_struct.buffer.start_iter(),
                &main_struct.buffer.end_iter(),
                false,
            ),
        ) {
            Ok(_) => {}
            Err(_) => {}
        },
        Message::SaveFile => {
            if let Ok(_) = exists(&main_struct.current_file_path) {
                // The program will attempt to save file, falling back to "Save As"
                // if it can't create the file from the current file path
                if let Ok(mut file) = File::create(&main_struct.current_file_path) {
                    file.write_all(
                        main_struct
                            .buffer
                            .text(
                                &main_struct.buffer.start_iter(),
                                &main_struct.buffer.end_iter(),
                                false,
                            )
                            .as_bytes(),
                    )
                    .unwrap();
                } else {
                    sender.input(Message::SaveAsRequest);
                }
            }
        }
        // Edit
        Message::ClearEditor => {
            main_struct.buffer.set_text("");
            main_struct.buffer.undo();
        }
        // View
        Message::ToggleFileList => {
            main_struct
                .side_bar_box
                .set_visible(!main_struct.side_bar_box.is_visible());
            save_settings(main_struct);
        }
        Message::ToggleHiddenFiles => {
            let current_folder = main_struct.current_folder_path.clone();
            main_struct.view_hidden = !main_struct.view_hidden;
            load_folder(main_struct, &current_folder);
            save_settings(main_struct);
        }
        Message::ToggleMiniMap => {
            main_struct
                .mini_map
                .set_visible(!main_struct.mini_map.is_visible());
            save_settings(main_struct);
        }
        Message::ToggleBufferStyleScheme => {
            match main_struct
                .buffer_style
                .as_ref()
                .unwrap()
                .to_string()
                .as_str()
            {
                "Adwaita Dark" => {
                    main_struct.buffer_style =
                        sourceview5::StyleSchemeManager::new().scheme("Adwaita");
                    main_struct
                        .buffer
                        .set_style_scheme(main_struct.buffer_style.as_ref());
                }
                "Adwaita" => {
                    main_struct.buffer_style =
                        sourceview5::StyleSchemeManager::new().scheme("Adwaita-dark");
                    main_struct
                        .buffer
                        .set_style_scheme(main_struct.buffer_style.as_ref());
                }
                _ => {}
            }
            save_settings(main_struct);
        }
        // About
        Message::ShowAbout => {
            crate::util::dialogs::create_about_dialog();
        }
        Message::ShowPreferences => {
            crate::util::dialogs::create_preferences_dialog(main_struct, sender);
        }
        // File list
        Message::FileListContext(x, y) => {
            let rect = Rectangle::new(x, y, 1, 1);
            main_struct
                .file_list_context_menu
                .set_pointing_to(Some(&rect));
            match main_struct.file_list.selected_row() {
                Some(_) => {
                    main_struct
                        .file_list_context_menu
                        .set_menu_model(Some(&file_list_context_menu_model_item()));
                    main_struct.file_list_context_menu.popup();
                }
                None => {
                    main_struct
                        .file_list_context_menu
                        .set_menu_model(Some(&file_list_context_menu_model()));
                    main_struct.file_list_context_menu.popup();
                }
            }
        }
        Message::DeleteItem => {
            if let Some(row) = main_struct.file_list.selected_row() {
                let mut file_list_pathbuf = PathBuf::from(&main_struct.current_folder_path);
                let file_list_name = &row.child().unwrap().widget_name();
                let file_list_path = Path::new(file_list_name);
                file_list_pathbuf.push(file_list_path);
                trash::delete(file_list_pathbuf).unwrap();
            }
            let path = main_struct.current_folder_path.clone();
            load_folder(main_struct, &path);
        }
        Message::OpenFolderExternal => {
            if let Ok(_) = fs::exists(&main_struct.current_folder_path) {
                if let Ok(_) = open::that(&main_struct.current_folder_path) {}
            }
        }
        // Other
        Message::LoadSettings => {
            println!("Loading Settings...");
            load_settings(main_struct);
        }
        Message::UpdateTabType(use_spaces) => {
            main_struct
                .editor
                .set_insert_spaces_instead_of_tabs(use_spaces);
            save_settings(main_struct);
        }
        Message::UpdateTabWidth(tab_width) => {
            main_struct.editor.set_tab_width(tab_width);
            save_settings(main_struct);
        }
        Message::UpDir => {
            if let Some(path) = PathBuf::from(&main_struct.current_folder_path).parent() {
                let up_dir = &path.to_str().unwrap().to_string();
                main_struct.current_folder_path = up_dir.clone();
                load_folder(main_struct, &up_dir);
            }
        }
        Message::RefreshFileList => {
            load_folder(main_struct, &main_struct.current_folder_path.clone());
        }
        Message::CursorPositionChanged => {
            if let Some(_) = main_struct.file_list.selected_row() {
                main_struct.file_list.unselect_all();
            }
        }
        Message::Ignore => {
            println!("Test");
        }
    }
}
