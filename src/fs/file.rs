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
                .add_toast(Toast::new(&state.current_file_path.display().to_string()));
        }
    }
}

pub fn save_file(main_struct: &mut State, sender: relm4::ComponentSender<State>) {
    if exists(&main_struct.current_file_path).is_ok() {
        // The program will attempt to save file, falling back to "Save As"
        // if it can't create the file from the current file path
        if let Ok(mut file) = File::create(&main_struct.current_file_path) {
            if file
                .write_all(
                    main_struct
                        .buffer
                        .text(
                            &main_struct.buffer.start_iter(),
                            &main_struct.buffer.end_iter(),
                            false,
                        )
                        .as_bytes(),
                )
                .is_err()
            {
                sender.input(Message::QuickToast("Error when saving file!".to_string()))
            }
        } else {
            sender.input(Message::SaveAsRequest);
        }
    }
}
