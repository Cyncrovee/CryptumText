use std::{
    fs::{DirEntry, read_dir, read_to_string},
    io::Error,
    path::{Path, PathBuf},
};

use git2::Repository;
use libadwaita::Toast;
use relm4::{RelmRemoveAllExt, gtk::prelude::*, prelude::*};
use sourceview5::prelude::{BufferExt, ViewExt};

use crate::{
    app::model::{AppSettings, MainStruct},
    util::widget::update_syntax,
};

pub fn load_file(self_from: &mut MainStruct) {
    match std::fs::read_to_string(&self_from.current_file_path) {
        Ok(f) => {
            self_from.buffer.set_text(&f);
            self_from.current_file_path = Some(self_from.current_file_path.clone()).unwrap();
            match update_syntax(&self_from.language_manager, &self_from.current_file_path) {
                Some(language) => {
                    self_from.buffer.set_highlight_syntax(true);
                    self_from.buffer.set_language(Some(&language));
                }
                None => {
                    self_from.buffer.set_highlight_syntax(false);
                }
            }
        }
        Err(_) => {
            self_from.toast_overlay.add_toast(Toast::new("Failed to Read File!"));
        }
    }
}

pub fn load_folder(self_from: &mut MainStruct, path: &String) {
    match read_dir(&path.clone()) {
        Ok(dir) => {
            self_from.file_list.remove_all();
            for files in dir {
                #[cfg(unix)]
                {
                    match self_from.view_hidden {
                        true => {
                            show_item(self_from, files);
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
                                show_item(self_from, files);
                            }
                        }
                    }
                }
                #[cfg(any(not(unix)))]
                {
                    show_item(self_from, files);
                }
            }
        }
        Err(_) => {
            self_from.toast_overlay.add_toast(Toast::new("Failed to Read Folder!"));
        }
    }
}

fn show_item(self_from: &mut MainStruct, files: Result<DirEntry, Error>) {
    let label = gtk::Label::builder().build();
    let dir_entry = files.as_ref().unwrap();
    label.set_widget_name(dir_entry.file_name().as_os_str().to_str().unwrap());
    let mut item_name = dir_entry.file_name().into_string().unwrap();
    if let true = PathBuf::from(dir_entry.path().into_os_string().into_string().unwrap()).is_dir() {
        item_name.push_str("/");
    }
    label.set_text(&item_name);
    self_from.file_list.append(&label);
    if let Ok(repo) = Repository::discover(&self_from.current_folder_path) {
        self_from.git_info.0 = repo.head().unwrap().shorthand().unwrap().to_string();
        self_from.git_info.1 = true;
    } else {
        self_from.git_info.0 = "".to_string();
        self_from.git_info.1 = false;
    }
}

pub fn save_settings(self_from: &mut MainStruct) {
    let mut config_path = dirs::config_dir().unwrap();
    config_path.push(Path::new("cryptum-text-settings.json"));

    let settings = AppSettings {
        editor_monospace: self_from.editor.is_monospace(),
        editor_theme: self_from.buffer_style.as_ref().unwrap().to_string(),
        editor_use_spaces_for_tabs: self_from.editor.is_insert_spaces_instead_of_tabs(),
        editor_tab_width: self_from.editor.tab_width(),
        view_mini_map: self_from.mini_map.is_visible(),
        view_file_list: self_from.file_list.is_visible(),
        view_hidden_files: self_from.view_hidden,
    };

    serde_json::to_string(&settings).unwrap();
    std::fs::write(
        config_path,
        serde_json::to_string_pretty(&settings).unwrap(),
    )
    .unwrap();
}

pub fn load_settings(self_from: &mut MainStruct) {
    let mut config_path = dirs::config_dir().unwrap();
    config_path.push(Path::new("cryptum-text-settings.json"));

    if let Err(_) = read_to_string(&config_path) {
        std::fs::write(&config_path, "").unwrap()
    }
    let settings_file = read_to_string(&config_path).unwrap();

    let settings: AppSettings = serde_json::from_str(&settings_file).unwrap_or(AppSettings {
        editor_monospace: true,
        editor_theme: self_from.buffer_style.as_ref().unwrap().to_string(),
        editor_use_spaces_for_tabs: true,
        editor_tab_width: 4,
        view_mini_map: true,
        view_file_list: true,
        view_hidden_files: false,
    });
    match settings.editor_theme.as_str() {
        "Adwaita" => {
            self_from.buffer_style = sourceview5::StyleSchemeManager::new().scheme("Adwaita");
            self_from
                .buffer
                .set_style_scheme(self_from.buffer_style.as_ref());
        }
        "Adwaita Dark" => {
            self_from.buffer_style = sourceview5::StyleSchemeManager::new().scheme("Adwaita-dark");
            self_from
                .buffer
                .set_style_scheme(self_from.buffer_style.as_ref());
        }
        &_ => {}
    }
    match settings.editor_monospace {
        true => {
            self_from.editor.set_monospace(true);
        }
        false => {
            self_from.editor.set_monospace(false);
        }
    }
    match settings.view_file_list {
        true => {
            self_from.side_bar_box.set_visible(true);
        }
        false => {
            self_from.side_bar_box.set_visible(false);
        }
    }
    match settings.view_mini_map {
        true => {
            self_from.mini_map.set_visible(true);
        }
        false => {
            self_from.mini_map.set_visible(false);
        }
    }
    match settings.view_hidden_files {
        true => {
            self_from.view_hidden = true;
        }
        false => {
            self_from.view_hidden = false;
        }
    }
    self_from
        .editor
        .set_insert_spaces_instead_of_tabs(settings.editor_use_spaces_for_tabs);
    self_from.editor.set_tab_width(settings.editor_tab_width);
}
