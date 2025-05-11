use std::{
    fs::{read_dir, read_to_string},
    path::Path,
};

use gtk4::ListBox;
use relm4::{RelmRemoveAllExt, gtk::prelude::*, prelude::*};
use sourceview5::{prelude::BufferExt, subclass::buffer};

use crate::{
    program_model::{AppSettings, MainStruct},
    widget_module::update_syntax,
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
        Err(_) => panic!("Failed to read file to string!"),
    }
}

pub fn load_folder(self_from: &mut MainStruct, path: &String) {
    match read_dir(&path.clone()) {
        Ok(dir) => {
            self_from.file_list.remove_all();
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
                label.set_text(files.unwrap().file_name().as_os_str().to_str().unwrap());
                self_from.file_list.append(&label);
            }
        }
        Err(_) => {
            println!("Failed to read directory");
        }
    }
}

pub fn save_settings(self_from: &mut MainStruct) {
    let mut config_path = dirs::config_dir().unwrap();
    config_path.push(Path::new("cryptum-text-settings.json"));

    let test = AppSettings {
        view_mini_map: self_from.mini_map.is_visible(),
        view_file_list: self_from.file_list.is_visible(),
    };

    serde_json::to_string(&test).unwrap();
    std::fs::write(config_path, serde_json::to_string_pretty(&test).unwrap()).unwrap();
}

pub fn load_settings(file_list: &ListBox, mini_map: &sourceview5::Map) {
    let mut config_path = dirs::config_dir().unwrap();
    config_path.push(Path::new("cryptum-text-settings.json"));

    match read_to_string(&config_path) {
        Ok(_) => {
            // Pass
        }
        Err(_) => std::fs::write(&config_path, "").unwrap(),
    }
    let settings_file = read_to_string(&config_path).unwrap();

    let settings: AppSettings = serde_json::from_str(&settings_file).unwrap_or(AppSettings {
        view_mini_map: true,
        view_file_list: true,
    });
    match settings.view_file_list {
        true => {
            file_list.set_visible(true);
        }
        false => {
            file_list.set_visible(false);
        }
    }
    match settings.view_mini_map {
        true => {
            // Pass
        }
        false => {
            mini_map.set_visible(false);
        }
    }
}
