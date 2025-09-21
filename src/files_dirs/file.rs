use std::{
    fs::{File, exists},
    io::Write,
};

use gtk4::prelude::TextBufferExt;
use libadwaita::Toast;
use sourceview5::prelude::BufferExt;

use crate::{
    app::model::{MainStruct, Message},
    util::widget::update_syntax,
};

pub fn load_file(main_struct: &mut MainStruct) {
    match std::fs::read_to_string(&main_struct.current_file_path) {
        Ok(f) => {
            main_struct.buffer.set_text(&f);
            match update_syntax(
                &main_struct.language_manager,
                &main_struct.current_file_path,
            ) {
                Some(language) => {
                    main_struct.buffer.set_highlight_syntax(true);
                    main_struct.buffer.set_language(Some(&language));
                }
                None => {
                    main_struct.buffer.set_highlight_syntax(false);
                }
            }
        }
        Err(_) => {
            main_struct
                .toast_overlay
                .add_toast(Toast::new(&main_struct.current_file_path));
        }
    }
}

pub fn save_file(main_struct: &mut MainStruct, sender: relm4::ComponentSender<MainStruct>) {
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
