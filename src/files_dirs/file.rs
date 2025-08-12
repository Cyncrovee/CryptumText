use gtk4::prelude::TextBufferExt;
use libadwaita::Toast;
use sourceview5::prelude::BufferExt;

use crate::{app::model::MainStruct, util::widget::update_syntax};

pub fn load_file(main_struct: &mut MainStruct) {
    match std::fs::read_to_string(&main_struct.current_file_path) {
        Ok(f) => {
            main_struct.buffer.set_text(&f);
            main_struct.current_file_path = main_struct.current_file_path.clone();
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
