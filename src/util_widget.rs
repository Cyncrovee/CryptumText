use std::path::PathBuf;

use gtk4::glib::GString;
use sourceview5::{Buffer, LanguageManager};

pub fn setup_editor(buffer: &Buffer) -> sourceview5::View {
    let editor = sourceview5::View::builder()
        .buffer(buffer)
        .hexpand(true)
        .vexpand(true)
        .smart_backspace(true)
        .monospace(true)
        .insert_spaces_instead_of_tabs(true)
        .highlight_current_line(true)
        .show_line_numbers(true)
        .auto_indent(true)
        .build();

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

pub fn update_file_type(file: &str) -> Option<GString> {
    let path = PathBuf::from(file);
    match path.extension() {
        Some(e) => match e.to_str() {
            Some(ex) => match ex {
                // Text/Config/Markup Files
                "txt" => {
                    return Some("Text File".into());
                }
                "md" => {
                    return Some("Markdown File".into());
                }
                "html" => {
                    return Some("HTML File".into());
                }
                "css" => {
                    return Some("CSS File".into());
                }
                "hbs" => {
                    return Some("Handlebars File".into());
                }
                "hxml" => {
                    return Some("Haxe Build File".into());
                }
                "xml" => {
                    return Some("XML File".into());
                }
                "xaml" => {
                    return Some("XAML File".into());
                }
                "axaml" => {
                    return Some("AXAML File".into());
                }
                "org" => {
                    return Some("Org Mode File".into());
                }
                "norg" => {
                    return Some("Neorg File".into());
                }
                "ini" => {
                    return Some("INI File".into());
                }
                "toml" => {
                    return Some("TOML File".into());
                }
                "json" => {
                    return Some("JSON File".into());
                }
                "jsonc" => {
                    return Some("JSONC File".into());
                }
                // Shell Files
                "sh" => {
                    return Some("Shell Script".into());
                }
                "ps1" => {
                    return Some("PowerShell Script".into());
                }
                "fish" => {
                    return Some("Fish Script".into());
                }
                // Source Files
                "rs" => {
                    return Some("Rust Source File".into());
                }
                "cr" => {
                    return Some("Crystal Source File".into());
                }
                "elm" => {
                    return Some("Elm Source File".into());
                }
                "ex" => {
                    return Some("Elixir Source File".into());
                }
                "exs" => {
                    return Some("Elixir lource File".into());
                }
                "gd" => {
                    return Some("GDScript Source File".into());
                }
                "rb" => {
                    return Some("Ruby Source File".into());
                }
                "py" => {
                    return Some("Python Source File".into());
                }
                "lua" => {
                    return Some("Lua Source File".into());
                }
                "c" => {
                    return Some("C Source File".into());
                }
                "ml" => {
                    return Some("OCaml Source File".into());
                }
                "cs" => {
                    return Some("C# Source File".into());
                }
                "php" => {
                    return Some("PHP Source File".into());
                }
                "ts" => {
                    return Some("TypeScript Source File".into());
                }
                "js" => {
                    return Some("JavaScript Source File".into());
                }
                "jl" => {
                    return Some("Julia Source File".into());
                }
                "lisp" => {
                    return Some("Common Lisp Source File".into());
                }
                "el" => {
                    return Some("ELisp Source File".into());
                }
                "erl" => {
                    return Some("Erlang Source File".into());
                }
                "hrl" => {
                    return Some("Erlang Header File".into());
                }
                "h" => {
                    return Some("Header File".into());
                }
                "hx" => {
                    return Some("Haxe Source File".into());
                }
                "v" => {
                    return Some("V Source File".into());
                }
                "vim" => {
                    return Some("Vimscript File".into());
                }
                "vimrc" => {
                    return Some("Vimscript File".into());
                }
                "vala" => {
                    return Some("Vala Source File".into());
                }
                "zig" => {
                    return Some("Zig Source File".into());
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
