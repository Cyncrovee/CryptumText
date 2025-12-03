use std::path::PathBuf;

use gtk4::{glib::GString, prelude::WidgetExt};
use sourceview5::{Buffer, LanguageManager, prelude::BufferExt};

use crate::{
    app::model::{ItemVis, State},
    fs::settings::save_settings,
};

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

pub(crate) fn toggle_buffer_style(state: &mut State) {
    match state.buffer_style.as_ref().unwrap().to_string().as_str() {
        "Adwaita Dark" => {
            state.buffer_style = sourceview5::StyleSchemeManager::new().scheme("Adwaita");
            state.buffer.set_style_scheme(state.buffer_style.as_ref());
        }
        "Adwaita" => {
            state.buffer_style = sourceview5::StyleSchemeManager::new().scheme("Adwaita-dark");
            state.buffer.set_style_scheme(state.buffer_style.as_ref());
        }
        _ => {}
    }
    save_settings(state);
}

pub(crate) fn update_vis(item: ItemVis, vis: bool, state: &mut State) {
    match item {
        ItemVis::MiniMap => {
            state.mini_map.set_visible(vis);
        }
        ItemVis::HiddenFiles => {
            state.view_hidden = vis;
        }
    }
    save_settings(state);
}

pub fn update_syntax(
    language_manager: &LanguageManager,
    current_file_path: &String,
) -> Option<sourceview5::Language> {
    language_manager.guess_language(Some(&current_file_path), None)
}

pub fn update_file_type(file: &str) -> Option<GString> {
    if let Some(ext) = PathBuf::from(file).extension()
        && let Some(ext_str) = ext.to_str()
    {
        match ext_str {
            // Text/Config/Markup Files
            "axaml" => Some("AXAML".into()),
            "base" => Some("Obsidian Base File".into()),
            "css" => Some("CSS".into()),
            "hbs" => Some("Handlebars".into()),
            "html" => Some("HTML".into()),
            "hxml" => Some("Haxe Build File".into()),
            "ini" => Some("INI".into()),
            "json" => Some("JSON".into()),
            "jsonc" => Some("JSONC".into()),
            "md" => Some("Markdown".into()),
            "norg" => Some("Neorg".into()),
            "org" => Some("Org Mode File".into()),
            "toml" => Some("TOML".into()),
            "txt" => Some("Text File".into()),
            "xaml" => Some("XAML".into()),
            "xml" => Some("XML".into()),
            // Shell Files
            "fish" => Some("Fish Script".into()),
            "ps1" => Some("PowerShell Script".into()),
            "sh" => Some("Shell Script".into()),
            // Source Files
            "c" => Some("C".into()),
            "cr" => Some("Crystal".into()),
            "cs" => Some("C#".into()),
            "el" => Some("ELisp".into()),
            "elm" => Some("Elm".into()),
            "erl" => Some("Erlang".into()),
            "ex" => Some("Elixir".into()),
            "exs" => Some("Elixir".into()),
            "gd" => Some("GDScript".into()),
            "gleam" => Some("Gleam".into()),
            "h" => Some("Header File".into()),
            "hrl" => Some("Erlang Header".into()),
            "hx" => Some("Haxe".into()),
            "java" => Some("Java".into()),
            "jl" => Some("Julia".into()),
            "js" => Some("JavaScript".into()),
            "kt" => Some("Kotlin".into()),
            "lisp" => Some("Common Lisp".into()),
            "lua" => Some("Lua".into()),
            "ml" => Some("OCaml".into()),
            "php" => Some("PHP".into()),
            "py" => Some("Python".into()),
            "rb" => Some("Ruby".into()),
            "rs" => Some("Rust".into()),
            "ts" => Some("TypeScript".into()),
            "v" => Some("V".into()),
            "vala" => Some("Vala".into()),
            "vim" => Some("Vimscript".into()),
            "vimrc" => Some("Vimscript".into()),
            "zig" => Some("Zig".into()),
            &_ => None,
        }
    } else {
        None
    }
}
