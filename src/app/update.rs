use std::path::PathBuf;

use gtk4::{
    gio::{File, FileType},
    prelude::*,
};
use relm4::ComponentController;
use relm4_components::{open_dialog::OpenDialogMsg, save_dialog::SaveDialogMsg};
use sourceview5::prelude::ViewExt;

use crate::{
    app::model::{Message, State},
    fs::{
        file::{load_file, save_file},
        folder::load_folder_view,
        settings::{load_settings, save_settings},
    },
    util::widget::{toggle_buffer_style, update_vis},
};

pub(crate) fn handle_messages(
    state: &mut State,
    message: Message,
    sender: relm4::ComponentSender<State>,
) {
    match message {
        // File
        Message::NewFile => {
            state.buffer.set_text("");
            state.current_file_path = PathBuf::new();
        }
        Message::FolderRequest => state.folder_dialog.emit(OpenDialogMsg::Open),
        Message::FolderResponse(path) => {
            state.current_folder_path = path;
            load_folder_view(state, sender);
        }
        Message::OpenRequest => state.open_dialog.emit(OpenDialogMsg::Open),
        Message::OpenResponse(path) => {
            state.current_file_path = path;
            load_file(state);
        }
        Message::SaveAsRequest => state
            .save_as_dialog
            .emit(SaveDialogMsg::SaveAs("".to_string())),
        Message::SaveAsResponse(path) => {
            _ = std::fs::write(
                &path,
                state
                    .buffer
                    .text(&state.buffer.start_iter(), &state.buffer.end_iter(), false),
            )
        }
        Message::SaveFile => {
            save_file(state, sender);
        }
        // Edit
        Message::ClearEditor => {
            state.buffer.set_text("");
            state.buffer.undo();
        }
        // View
        Message::ToggleFileList => {
            state
                .side_bar_box
                .set_visible(!state.side_bar_box.is_visible());
            save_settings(state);
        }
        Message::ToggleHiddenFiles => {
            state.view_hidden = !state.view_hidden;
            save_settings(state);
        }
        Message::ToggleMiniMap => {
            state.mini_map.set_visible(!state.mini_map.is_visible());
            save_settings(state);
        }
        Message::ToggleBufferStyleScheme => {
            toggle_buffer_style(state);
        }
        Message::ToggleFullscreen => match state.root.is_fullscreen() {
            true => {
                state.root.set_fullscreened(false);
            }
            false => {
                state.root.set_fullscreened(true);
            }
        },
        // About
        Message::ShowKeyboardShortcuts => {
            crate::util::dialogs::create_keyboard_shortcut_dialog();
        }
        Message::ShowPreferences => {
            crate::util::dialogs::create_preferences_dialog(state, sender);
        }
        Message::ShowAbout => {
            crate::util::dialogs::create_about_dialog();
        }
        // File list
        Message::LoadFileFromTree(file_info) => {
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
        Message::LoadSettings => {
            println!("Loading Settings...");
            load_settings(state);
        }
        Message::UpdateMonospace(value) => {
            state.editor.set_monospace(value);
            save_settings(state);
        }
        Message::UpdateTabType(use_spaces) => {
            state.editor.set_insert_spaces_instead_of_tabs(use_spaces);
            save_settings(state);
        }
        Message::UpdateTabWidth(tab_width) => {
            state.editor.set_tab_width(tab_width);
            save_settings(state);
        }
        Message::UpdateVisibility(item, vis) => {
            update_vis(item, vis, state);
        }
        Message::CursorPositionChanged => {}
        Message::Ignore => {}
    }
}
