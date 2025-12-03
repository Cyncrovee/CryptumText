use std::{fs::read_to_string, path::Path};

use gtk4::prelude::*;
use sourceview5::prelude::*;

use crate::app::model::{Settings, State};

pub fn save_settings(state: &mut State) {
    let mut config_path = dirs::config_dir().unwrap();
    config_path.push(Path::new("cryptum-text-settings.json"));

    std::fs::write(
        config_path,
        serde_json::to_string_pretty(&Settings {
            editor_monospace: state.editor.is_monospace(),
            editor_theme: state.buffer_style.as_ref().unwrap().to_string(),
            editor_use_spaces_for_tabs: state.editor.is_insert_spaces_instead_of_tabs(),
            editor_tab_width: state.editor.tab_width(),
            view_sidebar: state.nav_view.shows_sidebar(),
            view_mini_map: state.mini_map.is_visible(),
            view_hidden_files: state.view_hidden,
        })
        .unwrap(),
    )
    .unwrap();
}

pub fn load_settings(state: &mut State) {
    let mut config_path = dirs::config_dir().unwrap();
    config_path.push(Path::new("cryptum-text-settings.json"));

    if read_to_string(&config_path).is_err() {
        std::fs::write(&config_path, "").unwrap()
    }
    let settings_file = read_to_string(&config_path).unwrap();

    let settings: Settings = serde_json::from_str(&settings_file).unwrap_or(Settings {
        editor_theme: state.buffer_style.as_ref().unwrap().to_string(),
        ..Default::default()
    });
    match settings.editor_theme.as_str() {
        "Adwaita" => {
            state.buffer_style = sourceview5::StyleSchemeManager::new().scheme("Adwaita");
            state.buffer.set_style_scheme(state.buffer_style.as_ref());
        }
        "Adwaita Dark" => {
            state.buffer_style = sourceview5::StyleSchemeManager::new().scheme("Adwaita-dark");

        }
        &_ => {}
    }
    state.editor.set_monospace(settings.editor_monospace);
    state.nav_view.set_show_sidebar(settings.view_sidebar);
    state.mini_map.set_visible(settings.view_mini_map);
    state.view_hidden = settings.view_hidden_files;
    state
        .editor
        .set_insert_spaces_instead_of_tabs(settings.editor_use_spaces_for_tabs);
    state.editor.set_tab_width(settings.editor_tab_width);
}
