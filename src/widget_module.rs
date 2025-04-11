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
