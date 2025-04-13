use gtk4::prelude::TextBufferExt;
use sourceview5::prelude::BufferExt;

use crate::{program_model::MainStruct, widget_module::update_syntax};

pub fn load_file(self_from: &mut MainStruct) {
    match std::fs::read_to_string(&self_from.current_file_path) {
        Ok(f) => {
            self_from.buffer.set_text(&f);
            self_from.current_file_path = Some(self_from.current_file_path.clone()).unwrap();
            match update_syntax(&self_from.language_manager, &self_from.current_file_path) {
                Some(language) => {
                    self_from.buffer.set_language(Some(&language));
                }
                None => {
                    //
                }
            }
        }
        Err(_) => panic!("Failed to read file to string!"),
    }
}
