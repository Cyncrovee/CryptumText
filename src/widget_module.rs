use std::path::PathBuf;

use gtk4::glib::GString;
use relm4::RelmWidgetExt;
use sourceview5::{
    Buffer, LanguageManager,
    prelude::{TextViewExt, ViewExt},
};

pub fn setup_editor(buffer: &Buffer) -> sourceview5::View {
    let editor = sourceview5::View::builder().build();
    editor.set_buffer(Some(buffer));
    editor.set_expand(true);
    editor.set_smart_backspace(true);
    editor.set_monospace(true);
    editor.set_insert_spaces_instead_of_tabs(true);
    editor.set_highlight_current_line(true);
    editor.set_show_line_numbers(true);
    editor.set_indent_width(4);
    editor.set_auto_indent(true);

    return editor;
}

pub fn update_file_list() {
    //
}

pub fn update_syntax(
    language_manager: &LanguageManager,
    current_file_path: &String,
) -> Option<sourceview5::Language> {
    match language_manager.guess_language(Some(&current_file_path), None) {
        Some(language) => {
            return Some(language);
        }
        None => None,
    }
}

pub fn update_file_type(file: &str) -> Option<GString> {
    let path = PathBuf::from(file);
    match path.extension() {
        Some(e) => match e.to_str() {
            Some(ex) => match ex {
                "txt" => {
                    return Some("Text File".into());
                }
                "toml" => {
                    return Some("TOML File".into());
                }
                "json" => {
                    return Some("JSON File".into());
                }
                "rs" => {
                    return Some("Rust Source File".into());
                }
                "py" => {
                    return Some("Python Source File".into());
                }
                "cs" => {
                    return Some("C# Source File".into());
                }
                "ts" => {
                    return Some("TypeScript Source File".into());
                }
                "js" => {
                    return Some("JavaScript  Source File".into());
                }
                &_ => {
                    return None;
                }
            },
            None => None,
        },
        None => None,
    }
}
