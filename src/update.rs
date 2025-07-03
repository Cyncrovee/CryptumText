use std::{
    fs::{File, exists},
    io::Write,
    path::{Path, PathBuf},
};

use gtk4::{AboutDialog, prelude::*};
use relm4::ComponentController;
use relm4_components::{open_dialog::OpenDialogMsg, save_dialog::SaveDialogMsg};
use sourceview5::prelude::BufferExt;

use crate::{
    fs_module::{load_file, load_folder, load_settings, save_settings},
    program_model::{MainStruct, Message},
};

pub(crate) fn handle_messages(
    main_struct: &mut MainStruct,
    message: Message,
    _sender: relm4::ComponentSender<MainStruct>,
) {
    match message {
        // File
        Message::NewFile => {
            main_struct.buffer.set_text("");
            main_struct.current_file_path = "".to_string();
        }
        Message::LoadFileFromList => {
            if let Some(_) = main_struct.file_list.selected_row() {
                let mut file_list_pathbuf = PathBuf::from(&main_struct.current_folder_path);
                let file_list_name = &main_struct
                    .file_list
                    .selected_row()
                    .unwrap()
                    .child()
                    .unwrap()
                    .widget_name();
                let file_list_path = Path::new(file_list_name);
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
            Ok(_) => {
                // Pass
            }
            Err(_) => {
                // Pass
            }
        },
        Message::SaveFile => {
            if let Ok(_) = exists(&main_struct.current_file_path) {
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
                .file_list
                .set_visible(!main_struct.file_list.is_visible());
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
                _ => {
                    // Pass
                }
            }
            save_settings(main_struct);
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
        Message::FileListContext => {
            //
        }
        Message::LoadSettings => {
            println!("Loading Settings...");
            load_settings(main_struct);
        }
        Message::UpDir => {
            if let Some(_) = PathBuf::from(&main_struct.current_folder_path).parent() {
                let up_dir = PathBuf::from(&main_struct.current_folder_path)
                    .parent()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string();
                main_struct.current_folder_path = up_dir.clone();
                load_folder(main_struct, &up_dir);
            }
        }
        Message::CursorPostitionChanged => {
            // Pass
        }
        Message::Ignore => {
            // Pass
        }
    }
}
