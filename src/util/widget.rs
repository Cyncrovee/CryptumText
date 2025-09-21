use std::path::PathBuf;

use gtk4::glib::GString;
use sourceview5::{Buffer, LanguageManager};

pub fn setup_editor(buffer: &Buffer) -> sourceview5::View {
    sourceview5::View::builder()
        .buffer(buffer)
        .hexpand(true)
        .vexpand(true)
        .smart_backspace(true)
        .monospace(true)
        .highlight_current_line(true)
        .show_line_numbers(true)
        .auto_indent(true)
        .build()
}

pub fn update_syntax(
    language_manager: &LanguageManager,
    current_file_path: &String,
) -> Option<sourceview5::Language> {
    language_manager.guess_language(Some(&current_file_path), None)
}

pub fn update_file_type(file: &str) -> Option<GString> {
    let path = PathBuf::from(file);
    match path.extension() {
        Some(e) => match e.to_str() {
            Some(ex) => match ex {
                // Text/Config/Markup Files
                "txt" => Some("Text File".into()),
                "md" => Some("Markdown File".into()),
                "html" => Some("HTML File".into()),
                "css" => Some("CSS File".into()),
                "hbs" => Some("Handlebars File".into()),
                "hxml" => Some("Haxe Build File".into()),
                "xml" => Some("XML File".into()),
                "xaml" => Some("XAML File".into()),
                "axaml" => Some("AXAML File".into()),
                "org" => Some("Org Mode File".into()),
                "norg" => Some("Neorg File".into()),
                "ini" => Some("INI File".into()),
                "toml" => Some("TOML File".into()),
                "json" => Some("JSON File".into()),
                "jsonc" => Some("JSONC File".into()),
                "base" => Some("Obsidian Base File".into()),
                // Shell Files
                "sh" => Some("Shell Script".into()),
                "ps1" => Some("PowerShell Script".into()),
                "fish" => Some("Fish Script".into()),
                // Source Files
                "rs" => Some("Rust Source File".into()),
                "cr" => Some("Crystal Source File".into()),
                "elm" => Some("Elm Source File".into()),
                "ex" => Some("Elixir Source File".into()),
                "exs" => Some("Elixir lource File".into()),
                "gd" => Some("GDScript Source File".into()),
                "rb" => Some("Ruby Source File".into()),
                "py" => Some("Python Source File".into()),
                "lua" => Some("Lua Source File".into()),
                "c" => Some("C Source File".into()),
                "ml" => Some("OCaml Source File".into()),
                "cs" => Some("C# Source File".into()),
                "php" => Some("PHP Source File".into()),
                "ts" => Some("TypeScript Source File".into()),
                "js" => Some("JavaScript Source File".into()),
                "jl" => Some("Julia Source File".into()),
                "lisp" => Some("Common Lisp Source File".into()),
                "el" => Some("ELisp Source File".into()),
                "erl" => Some("Erlang Source File".into()),
                "hrl" => Some("Erlang Header File".into()),
                "h" => Some("Header File".into()),
                "hx" => Some("Haxe Source File".into()),
                "v" => Some("V Source File".into()),
                "vim" => Some("Vimscript File".into()),
                "vimrc" => Some("Vimscript File".into()),
                "vala" => Some("Vala Source File".into()),
                "zig" => Some("Zig Source File".into()),
                &_ => None,
            },
            None => None,
        },
        None => None,
    }
}
