use std::{
    fs::{DirEntry, read_dir},
    io::Error,
};

use git2::Repository;
use gtk4::{gdk::ffi::GDK_BUTTON_PRIMARY, glib::clone};
use libadwaita::Toast;
use relm4::{RelmRemoveAllExt, gtk::prelude::*, prelude::*};

use crate::app::model::{MainStruct, Message};

pub fn load_folder(main_struct: &mut MainStruct, sender: relm4::ComponentSender<MainStruct>) {
    match read_dir(&main_struct.current_folder_path) {
        Ok(dir) => {
            main_struct.file_list.remove_all();
            for files in dir {
                #[cfg(unix)]
                {
                    match main_struct.view_hidden {
                        true => {
                            show_item(main_struct, files, &sender);
                        }
                        false => {
                            if !files
                                .as_ref()
                                .unwrap()
                                .file_name()
                                .to_str()
                                .unwrap()
                                .starts_with(['.'])
                            {
                                show_item(main_struct, files, &sender);
                            }
                        }
                    }
                }
                #[cfg(any(not(unix)))]
                {
                    show_item(main_struct, files);
                }
            }
        }
        Err(_) => {
            main_struct
                .toast_overlay
                .add_toast(Toast::new("Failed to Read Folder!"));
        }
    }
}

fn show_item(
    main_struct: &mut MainStruct,
    files: Result<DirEntry, Error>,
    sender: &relm4::ComponentSender<MainStruct>,
) {
    if let Ok(entry) = files {
        if let Ok(entry_data) = entry.metadata() {
            match entry_data.is_file() {
                true => show_file(main_struct, entry),
                false => show_dir(main_struct, entry, sender),
            }
        } else {
            match entry.path().into_os_string().into_string() {
                Ok(path) => {
                    sender.input(Message::QuickToast(format!(
                        "Failed to get metadata for: {}",
                        path
                    )));
                }
                Err(_) => {
                    sender.input(Message::QuickToast(
                        "Failed to get metadata for DirEntry!".to_string(),
                    ));
                }
            }
        }
    } else {
        sender.input(Message::QuickToast(
            "Result<DirEntry, Error> returned Error!".to_string(),
        ));
    }
}

fn show_file(main_struct: &mut MainStruct, entry: DirEntry) {
    let dir_entry = entry;
    let item_name = dir_entry.file_name().into_string().unwrap();
    let label = gtk::Label::builder()
        .name(dir_entry.path().into_os_string().into_string().unwrap())
        .label(&item_name)
        .build();
    main_struct.file_list.append(&label);
    if let Ok(repo) = Repository::discover(&main_struct.current_folder_path) {
        main_struct.git_info.0 = repo.head().unwrap().shorthand().unwrap().to_string();
        main_struct.git_info.1 = true;
    } else {
        main_struct.git_info.0 = "".to_string();
        main_struct.git_info.1 = false;
    }
}

fn show_dir(
    main_struct: &mut MainStruct,
    entry: DirEntry,
    sender: &relm4::ComponentSender<MainStruct>,
) {
    let mut dir_label_text = entry.file_name().to_owned();
    dir_label_text.push("/");
    let dir_label = gtk::Label::new(Some(dir_label_text.as_os_str().to_str().unwrap()));
    let mut local_file_vec = Vec::default();
    for dir_entry in read_dir(entry.path()).expect("Failed to read directory!") {
        if dir_entry
            .as_ref()
            .expect("Failed to get DirEntry as reference!")
            .metadata()
            .expect("Failed to get item metadata!")
            .is_file()
        {
            let mut label_hidden_name = dir_label_text.clone().into_string().unwrap();
            label_hidden_name.push_str(
                &dir_entry
                    .as_ref()
                    .unwrap()
                    .file_name()
                    .into_string()
                    .unwrap(),
            );
            let local_dir_entry = dir_entry.as_ref();
            let label = gtk::Label::builder()
                .name(
                    local_dir_entry
                        .unwrap()
                        .path()
                        .into_os_string()
                        .into_string()
                        .unwrap(),
                )
                .label(dir_entry.unwrap().file_name().as_os_str().to_str().unwrap())
                .build();
            local_file_vec.push(label);
        }
    }
    main_struct.file_list.append(&dir_label);
    for label in local_file_vec.clone() {
        main_struct.file_list.append(&label);
        label.parent().unwrap().set_visible(false);
    }
    let dir_label_gesture = gtk::GestureClick::builder()
        .button(GDK_BUTTON_PRIMARY as u32)
        .build();
    dir_label_gesture.connect_released(clone!(
        #[strong]
        sender,
        move |_, _, _, _| sender.input(Message::ExpandLocalList(local_file_vec.clone()))
    ));
    dir_label.add_controller(dir_label_gesture);
}
