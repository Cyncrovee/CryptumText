use std::{fs::read_to_string, path::Path};

use gtk4::prelude::*;
use sourceview5::prelude::*;

use crate::app::model::{AppSettings, MainStruct};

pub fn save_settings(main_struct: &mut MainStruct) {
    let mut config_path = dirs::config_dir().unwrap();
    config_path.push(Path::new("cryptum-text-settings.json"));

    std::fs::write(
        config_path,
        serde_json::to_string_pretty(&AppSettings {
            editor_monospace: main_struct.editor.is_monospace(),
            editor_theme: main_struct.buffer_style.as_ref().unwrap().to_string(),
            editor_use_spaces_for_tabs: main_struct.editor.is_insert_spaces_instead_of_tabs(),
            editor_tab_width: main_struct.editor.tab_width(),
            view_mini_map: main_struct.mini_map.is_visible(),
            view_hidden_files: main_struct.view_hidden,
        })
        .unwrap(),
    )
    .unwrap();
}

pub fn load_settings(main_struct: &mut MainStruct) {
    let mut config_path = dirs::config_dir().unwrap();
    config_path.push(Path::new("cryptum-text-settings.json"));

    if read_to_string(&config_path).is_err() {
        std::fs::write(&config_path, "").unwrap()
    }
    let settings_file = read_to_string(&config_path).unwrap();

    let settings: AppSettings = serde_json::from_str(&settings_file).unwrap_or(AppSettings {
        editor_theme: main_struct.buffer_style.as_ref().unwrap().to_string(),
        ..Default::default()
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
    main_struct.editor.set_monospace(settings.editor_monospace);
    main_struct.mini_map.set_visible(settings.view_mini_map);
    main_struct.view_hidden = settings.view_hidden_files;
    main_struct
        .editor
        .set_insert_spaces_instead_of_tabs(settings.editor_use_spaces_for_tabs);
    main_struct.editor.set_tab_width(settings.editor_tab_width);
}
