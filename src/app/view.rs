use gtk4::prelude::TextBufferExt;

use crate::{
    app::model::{MainStruct, WidgetStruct},
    util::widget::update_file_type,
};

pub(crate) fn handle_view(
    main_struct: &MainStruct,
    _widgets: &mut WidgetStruct,
    _sender: relm4::ComponentSender<MainStruct>,
) {
    main_struct
        .title
        .set_subtitle(&main_struct.current_folder_path);
    match update_file_type(&main_struct.current_file_path) {
        Some(file_type) => {
            main_struct.file_type_label.set_label(&file_type);
        }
        None => {
            main_struct.file_type_label.set_label("");
        }
    }
    let cursor_iter = &main_struct
        .buffer
        .iter_at_offset(main_struct.buffer.cursor_position());
    let cursor_line = cursor_iter.line();
    let cursor_row = cursor_iter.line_offset();
    let mut status_line_extras = cursor_line.to_string().to_owned();
    status_line_extras.push(':');
    status_line_extras.push_str(cursor_row.to_string().as_str());
    status_line_extras.push_str(" | ");
    status_line_extras.push_str(main_struct.git_info.0.as_str());
    main_struct
        .cursor_position_label
        .set_label(&status_line_extras);
}
