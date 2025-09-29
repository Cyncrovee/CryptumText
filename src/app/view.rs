use gtk4::prelude::TextBufferExt;

use crate::{
    app::model::{State, WidgetStruct},
    util::widget::update_file_type,
};

pub(crate) fn handle_view(
    state: &State,
    _widgets: &mut WidgetStruct,
    _sender: relm4::ComponentSender<State>,
) {
    state.title.set_subtitle(&state.current_folder_path);
    match update_file_type(&state.current_file_path) {
        Some(file_type) => {
            state
                .file_type_label
                .set_label(format!("   {}", &file_type).as_str());
        }
        None => {
            state.file_type_label.set_label("");
        }
    }
    let cursor_iter = &state.buffer.iter_at_offset(state.buffer.cursor_position());
    let cursor_line = cursor_iter.line();
    let cursor_row = cursor_iter.line_offset();
    state
        .cursor_position_label
        .set_label(format!("{}:{}   ", cursor_line, cursor_row).as_str());
}
