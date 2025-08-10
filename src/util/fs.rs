use std::{
    fs::{DirEntry, read_dir, read_to_string},
    io::Error,
    path::{Path, PathBuf},
};

use git2::Repository;
use gtk4::{TreeStore, TreeViewColumn};
use libadwaita::Toast;
use relm4::{RelmRemoveAllExt, gtk::prelude::*, prelude::*};
use sourceview5::prelude::{BufferExt, ViewExt};

use crate::{
    app::model::{AppSettings, MainStruct},
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
                .add_toast(Toast::new("Failed to Read File!"));
        }
    }
}

pub fn load_folder(main_struct: &mut MainStruct, path: &String) {
    match read_dir(&path.clone()) {
        Ok(dir) => {
            main_struct.file_list.remove_all();
            for files in dir {
                #[cfg(unix)]
                {
                    match main_struct.view_hidden {
                        true => {
                            show_item(main_struct, files);
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
                                show_item(main_struct, files);
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

pub fn load_folder_to_tree(main_struct: &mut MainStruct, path: &String) {
    match read_dir(&path.clone()) {
        Ok(dir) => {
            for col in main_struct.file_tree.columns() {
                main_struct.file_tree.remove_column(&col);
            }
            let model = TreeStore::new(&[String::static_type()]);
            let mut dir_col_num: u16 = 1;
            let col = gtk::TreeViewColumn::new();
            let cell = gtk::CellRendererText::new();
            col.pack_start(&cell, true);
            col.add_attribute(&cell, "text", 0);
            main_struct.file_tree.append_column(&col);
            for files in dir {
                let file = files.unwrap();
                match file.metadata().unwrap().is_file() {
                    true => {
                        model.insert_with_values(
                            None,
                            None,
                            &[(0, &file.file_name().to_os_string().into_string().unwrap())],
                        );
                    }
                    false => {
                        main_struct.file_tree.append_column(&TreeViewColumn::new());
                    }
                }
            }
            main_struct.file_tree.set_model(Some(&model));
        }
        Err(_) => {
            main_struct
                .toast_overlay
                .add_toast(Toast::new("Failed to Read Folder!"));
        }
    }
}
fn show_item(main_struct: &mut MainStruct, files: Result<DirEntry, Error>) {
    let label = gtk::Label::builder().build();
    let dir_entry = files.as_ref().unwrap();
    label.set_widget_name(dir_entry.file_name().as_os_str().to_str().unwrap());
    let mut item_name = dir_entry.file_name().into_string().unwrap();
    if let true = PathBuf::from(dir_entry.path().into_os_string().into_string().unwrap()).is_dir() {
        item_name.push_str("/");
    }
    label.set_text(&item_name);
    main_struct.file_list.append(&label);
    if let Ok(repo) = Repository::discover(&main_struct.current_folder_path) {
        main_struct.git_info.0 = repo.head().unwrap().shorthand().unwrap().to_string();
        main_struct.git_info.1 = true;
    } else {
        main_struct.git_info.0 = "".to_string();
        main_struct.git_info.1 = false;
    }
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
