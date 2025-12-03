use std::{
    fs::{File, exists},
    io::Write,
};

use gtk4::prelude::TextBufferExt;
use libadwaita::Toast;
use sourceview5::prelude::BufferExt;

use crate::{
    app::model::{Message, State},
    util::widget::update_syntax,
};

pub fn load_file(state: &mut State) {
    match std::fs::read_to_string(&state.current_file_path) {
        Ok(f) => {
            state.buffer.set_text(&f);
            match update_syntax(
                &state.language_manager,
                &state.current_file_path.display().to_string(),
            ) {
                Some(language) => {
                    state.buffer.set_highlight_syntax(true);
                    state.buffer.set_language(Some(&language));
                }
                None => {
                    state.buffer.set_highlight_syntax(false);
                }
            }
        }
        Err(_) => {
            state
                .toast_overlay
                .add_toast(Toast::new("Error when loading file!"));
        }
    }
}

/// The program will attempt to save file, falling back to "Save As"
/// if it can't create the file from the current file path.
pub fn save_file(state: &mut State, sender: relm4::ComponentSender<State>) {
    if exists(&state.current_file_path).is_ok() {
        if let Ok(mut file) = File::create(&state.current_file_path) {
            if file
                .write_all(
                    state
                        .buffer
                        .text(&state.buffer.start_iter(), &state.buffer.end_iter(), false)
                        .as_bytes(),
                )
                .is_err()
            {
                state
                    .toast_overlay
                    .add_toast(Toast::new("Error when saving file!"));
            }
        } else {
            sender.input(Message::SaveAsRequest);
        }
    }
}
