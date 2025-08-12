use std::{
    fs::{DirEntry, read_dir, read_to_string},
    io::Error,
    path::Path,
};

use git2::Repository;
use gtk4::{gdk::ffi::GDK_BUTTON_PRIMARY, glib::clone};
use libadwaita::Toast;
use relm4::{RelmRemoveAllExt, gtk::prelude::*, prelude::*};
use sourceview5::prelude::{BufferExt, ViewExt};

use crate::{
    app::model::{AppSettings, MainStruct, Message},
    util::widget::update_syntax,
};

pub fn load_file(main_struct: &mut MainStruct) {
    match std::fs::read_to_string(&main_struct.current_file_path) {
        Ok(f) => {
            main_struct.buffer.set_text(&f);
            main_struct.current_file_path = Some(main_struct.current_file_path.clone()).unwrap();
            match update_syntax(
                &main_struct.language_manager,
                &main_struct.current_file_path,
            ) {
                Some(language) => {
                    main_struct.buffer.set_highlight_syntax(true);
                    main_struct.buffer.set_language(Some(&language));
                }
                None => {
                    main_struct.buffer.set_highlight_syntax(false);
                }
            }
        }
        Err(_) => {
            main_struct
                .toast_overlay
                .add_toast(Toast::new(&main_struct.current_file_path));
        }
    }
}

pub fn load_folder(
    main_struct: &mut MainStruct,
    path: &String,
    sender: relm4::ComponentSender<MainStruct>,
) {
    match read_dir(&path.clone()) {
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
                            if let false = files
                                .as_ref()
                                .unwrap()
                                .file_name()
                                .to_str()
                                .unwrap()
                                .starts_with(&['.'])
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
                    sender.input(Message::QuickToast(format!(
                        "Failed to get metadata for DirEntry!",
                    )));
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
    let dir_label = gtk::Label::new(Some(&dir_label_text.as_os_str().to_str().unwrap()));
    let mut local_file_vec = Vec::default();
    for dir_entry in read_dir(entry.path()).expect("Failed to read directory!") {
        match dir_entry
            .as_ref()
            .expect("Failed to get DirEntry as reference!")
            .metadata()
            .expect("Failed to get item metadata!")
            .is_file()
        {
            true => {
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
            false => {}
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

pub fn save_settings(main_struct: &mut MainStruct) {
    let mut config_path = dirs::config_dir().unwrap();
    config_path.push(Path::new("cryptum-text-settings.json"));

    let settings = AppSettings {
        editor_monospace: main_struct.editor.is_monospace(),
        editor_theme: main_struct.buffer_style.as_ref().unwrap().to_string(),
        editor_use_spaces_for_tabs: main_struct.editor.is_insert_spaces_instead_of_tabs(),
        editor_tab_width: main_struct.editor.tab_width(),
        view_mini_map: main_struct.mini_map.is_visible(),
        view_file_list: main_struct.file_list.is_visible(),
        view_hidden_files: main_struct.view_hidden,
    };

    serde_json::to_string(&settings).unwrap();
    std::fs::write(
        config_path,
        serde_json::to_string_pretty(&settings).unwrap(),
    )
    .unwrap();
}

pub fn load_settings(main_struct: &mut MainStruct) {
    let mut config_path = dirs::config_dir().unwrap();
    config_path.push(Path::new("cryptum-text-settings.json"));

    if let Err(_) = read_to_string(&config_path) {
        std::fs::write(&config_path, "").unwrap()
    }
    let settings_file = read_to_string(&config_path).unwrap();

    let settings: AppSettings = serde_json::from_str(&settings_file).unwrap_or(AppSettings {
        editor_monospace: true,
        editor_theme: main_struct.buffer_style.as_ref().unwrap().to_string(),
        editor_use_spaces_for_tabs: true,
        editor_tab_width: 4,
        view_mini_map: true,
        view_file_list: true,
        view_hidden_files: false,
    });
    match settings.editor_theme.as_str() {
        "Adwaita" => {
            main_struct.buffer_style = sourceview5::StyleSchemeManager::new().scheme("Adwaita");
            main_struct
                .buffer
                .set_style_scheme(main_struct.buffer_style.as_ref());
        }
        "Adwaita Dark" => {
            main_struct.buffer_style =
                sourceview5::StyleSchemeManager::new().scheme("Adwaita-dark");
            main_struct
                .buffer
                .set_style_scheme(main_struct.buffer_style.as_ref());
        }
        &_ => {}
    }
    match settings.editor_monospace {
        true => {
            main_struct.editor.set_monospace(true);
        }
        false => {
            main_struct.editor.set_monospace(false);
        }
    }
    match settings.view_file_list {
        true => {
            main_struct.side_bar_box.set_visible(true);
        }
        false => {
            main_struct.side_bar_box.set_visible(false);
        }
    }
    match settings.view_mini_map {
        true => {
            main_struct.mini_map.set_visible(true);
        }
        false => {
            main_struct.mini_map.set_visible(false);
        }
    }
    match settings.view_hidden_files {
        true => {
            main_struct.view_hidden = true;
        }
        false => {
            main_struct.view_hidden = false;
        }
    }
    main_struct
        .editor
        .set_insert_spaces_instead_of_tabs(settings.editor_use_spaces_for_tabs);
    main_struct.editor.set_tab_width(settings.editor_tab_width);
}
