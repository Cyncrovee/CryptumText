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
        ItemVis::SideBar => {
            state.side_bar_box.set_visible(vis);
        }
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
    let path = PathBuf::from(file);
    match path.extension() {
        Some(e) => match e.to_str() {
            Some(ex) => match ex {
                // Text/Config/Markup Files
                "txt" => Some("Text File".into()),
                "md" => Some("Markdown".into()),
                "html" => Some("HTML".into()),
                "css" => Some("CSS".into()),
                "hbs" => Some("Handlebars".into()),
                "hxml" => Some("Haxe Build File".into()),
                "xml" => Some("XML".into()),
                "xaml" => Some("XAML".into()),
                "axaml" => Some("AXAML".into()),
                "org" => Some("Org Mode File".into()),
                "norg" => Some("Neorg".into()),
                "ini" => Some("INI".into()),
                "toml" => Some("TOML".into()),
                "json" => Some("JSON".into()),
                "jsonc" => Some("JSONC".into()),
                "base" => Some("Obsidian Base File".into()),
                // Shell Files
                "sh" => Some("Shell Script".into()),
                "ps1" => Some("PowerShell Script".into()),
                "fish" => Some("Fish Script".into()),
                // Source Files
                "rs" => Some("Rust".into()),
                "cr" => Some("Crystal".into()),
                "elm" => Some("Elm".into()),
                "ex" => Some("Elixir".into()),
                "exs" => Some("Elixir".into()),
                "gd" => Some("GDScript".into()),
                "rb" => Some("Ruby".into()),
                "py" => Some("Python".into()),
                "lua" => Some("Lua".into()),
                "c" => Some("C".into()),
                "ml" => Some("OCaml".into()),
                "cs" => Some("C#".into()),
                "php" => Some("PHP".into()),
                "ts" => Some("TypeScript".into()),
                "js" => Some("JavaScript".into()),
                "jl" => Some("Julia".into()),
                "lisp" => Some("Common Lisp".into()),
                "el" => Some("ELisp".into()),
                "erl" => Some("Erlang".into()),
                "hrl" => Some("Erlang Header".into()),
                "h" => Some("Header File".into()),
                "hx" => Some("Haxe".into()),
                "v" => Some("V".into()),
                "vim" => Some("Vimscript".into()),
                "vimrc" => Some("Vimscript".into()),
                "vala" => Some("Vala".into()),
                "zig" => Some("Zig".into()),
                &_ => None,
            },
            None => None,
        },
        None => None,
    }
}
