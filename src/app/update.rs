use std::path::PathBuf;

use gtk4::{
    gio::{File, FileType},
    prelude::*,
};
use relm4::ComponentController;
use relm4_components::{open_dialog::OpenDialogMsg, save_dialog::SaveDialogMsg};
use sourceview5::prelude::ViewExt;

use crate::{
    app::model::{Msg, State},
    fs::{
        file::{load_file, save_file},
        folder::load_folder,
        settings::{load_settings, save_settings},
    },
    util::widget::{toggle_buffer_style, update_vis},
};

pub(crate) fn handle_messages(
    state: &mut State,
    message: Msg,
    sender: relm4::ComponentSender<State>,
) {
    match message {
        // File
        Msg::NewFile => {
            state.buffer.set_text("");
            state.current_file_path = PathBuf::new();
        }
        Msg::FolderRequest => state.folder_dialog.emit(OpenDialogMsg::Open),
        Msg::FolderResponse(path) => {
            state.current_folder_path = path;
            load_folder(state, sender);
        }
        Msg::OpenRequest => state.open_dialog.emit(OpenDialogMsg::Open),
        Msg::OpenResponse(path) => {
            state.current_file_path = path;
            load_file(state);
        }
        Msg::SaveAsRequest => state
            .save_as_dialog
            .emit(SaveDialogMsg::SaveAs("".to_string())),
        Msg::SaveAsResponse(path) => {
            _ = std::fs::write(
                &path,
                state
                    .buffer
                    .text(&state.buffer.start_iter(), &state.buffer.end_iter(), false),
            )
        }
        Msg::SaveFile => {
            save_file(state, sender);
        }
        // Edit
        Msg::ClearEditor => {
            state.buffer.set_text("");
            state.buffer.undo();
        }
        // View
        Msg::ToggleFileTree => {
            state.nav_view.set_show_sidebar(!state.nav_view.shows_sidebar());
            save_settings(state);
        }
        Msg::ToggleHiddenFiles => {
            state.view_hidden = !state.view_hidden;
            save_settings(state);
        }
        Msg::ToggleMiniMap => {
            state.mini_map.set_visible(!state.mini_map.is_visible());
            save_settings(state);
        }
        Msg::ToggleBufferStyleScheme => {
            toggle_buffer_style(state);
        }
        Msg::ToggleFullscreen => state.root.set_fullscreened(!state.root.is_fullscreen()),
        // About
        Msg::ShowKeyboardShortcuts => {
            crate::util::dialogs::create_keyboard_shortcut_dialog();
        }
        Msg::ShowPreferences => {
            crate::util::dialogs::create_preferences_dialog(state, sender);
        }
        Msg::ShowAbout => {
            crate::util::dialogs::create_about_dialog();
        }
        // File list
        Msg::LoadFileFromTree(file_info) => {
            if file_info.file_type() == FileType::Regular
                && let Some(file) = file_info
                    .attribute_object("standard::file")
                    .and_downcast_ref::<File>()
                && let Some(path) = file.path()
            {
                state.current_file_path = path;
                load_file(state);
            }
        }
        // Other
        Msg::LoadSettings => {
            println!("Loading Settings...");
            load_settings(state);
        }
        Msg::UpdateMonospace(value) => {
            state.editor.set_monospace(value);
            save_settings(state);
        }
        Msg::UpdateTabType(use_spaces) => {
            state.editor.set_insert_spaces_instead_of_tabs(use_spaces);
            save_settings(state);
        }
        Msg::UpdateTabWidth(tab_width) => {
            state.editor.set_tab_width(tab_width);
            save_settings(state);
        }
        Msg::UpdateVisibility(item, vis) => {
            update_vis(item, vis, state);
        }
        Msg::CursorPositionChanged => {}
        Msg::Ignore => {}
    }
}
