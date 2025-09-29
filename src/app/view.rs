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
            state.file_type_label.set_label(&file_type);
        }
        None => {
            state.file_type_label.set_label("");
        }
    }
    let cursor_iter = &state.buffer.iter_at_offset(state.buffer.cursor_position());
    let cursor_line = cursor_iter.line();
    let cursor_row = cursor_iter.line_offset();
    let mut status_line_extras = cursor_line.to_string().to_owned();
    status_line_extras.push(':');
    status_line_extras.push_str(cursor_row.to_string().as_str());
    status_line_extras.push_str(" | ");
    status_line_extras.push_str(state.git_info.0.as_str());
    state.cursor_position_label.set_label(&status_line_extras);
}
