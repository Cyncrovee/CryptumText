use std::fs::read_dir;

use relm4::{RelmRemoveAllExt, gtk::prelude::*, prelude::*};
use sourceview5::prelude::BufferExt;

use crate::{program_model::MainStruct, widget_module::update_syntax};

pub fn load_file(self_from: &mut MainStruct) {
    match std::fs::read_to_string(&self_from.current_file_path) {
        Ok(f) => {
            self_from.buffer.set_text(&f);
            self_from.current_file_path = Some(self_from.current_file_path.clone()).unwrap();
            match update_syntax(&self_from.language_manager, &self_from.current_file_path) {
                Some(language) => {
                    self_from.buffer.set_highlight_syntax(true);
                    self_from.buffer.set_language(Some(&language));
                }
                None => {
                    self_from.buffer.set_highlight_syntax(false);
                }
            }
        }
        Err(_) => panic!("Failed to read file to string!"),
    }
}

pub fn load_folder(self_from: &mut MainStruct, path: &String) {
    match read_dir(&path.clone()) {
        Ok(dir) => {
            self_from.file_list.remove_all();
            for files in dir {
                let label = gtk::Label::builder().build();
                label.set_widget_name(
                    files
                        .as_ref()
                        .unwrap()
                        .file_name()
                        .as_os_str()
                        .to_str()
                        .unwrap(),
                );
                label.set_text(files.unwrap().file_name().as_os_str().to_str().unwrap());
                self_from.file_list.append(&label);
            }
        }
        Err(_) => {
            println!("Failed to read directory");
        }
    }
}
