use std::fs;

use gtk4::prelude::*;
use libadwaita::Toast;
use relm4::ComponentController;
use relm4_components::{open_dialog::OpenDialogMsg, save_dialog::SaveDialogMsg};
use sourceview5::prelude::ViewExt;

use crate::{
    app::model::{MainStruct, Message},
    fs::{
        file::{load_file, save_file},
        folder::load_folder_view,
        settings::{load_settings, save_settings},
    },
    util::widget::toggle_buffer_style,
};

use super::model::ItemVis;

pub(crate) fn handle_messages(
    main_struct: &mut MainStruct,
    message: Message,
    sender: relm4::ComponentSender<MainStruct>,
) {
    match message {
        // File
        Message::NewFile => {
            main_struct.buffer.set_text("");
            main_struct.current_file_path = "".to_string();
        }
        Message::FolderRequest => main_struct.folder_dialog.emit(OpenDialogMsg::Open),
        Message::FolderResponse(path) => {
            if let Ok(path_string) = path.into_os_string().into_string() {
                main_struct.current_folder_path = path_string;
                load_folder_view(main_struct);
            } else {
                sender.input(Message::QuickToast(
                    "Failed to convert OsString to String".to_string(),
                ))
            }
        }
        Message::OpenRequest => main_struct.open_dialog.emit(OpenDialogMsg::Open),
        Message::OpenResponse(path) => {
            main_struct.current_file_path = path.into_os_string().into_string().unwrap();
            load_file(main_struct);
        }
        Message::SaveAsRequest => main_struct
            .save_as_dialog
            .emit(SaveDialogMsg::SaveAs("".to_string())),
        Message::SaveAsResponse(path) => {
            if std::fs::write(
                &path,
                main_struct.buffer.text(
                    &main_struct.buffer.start_iter(),
                    &main_struct.buffer.end_iter(),
                    false,
                ),
            )
            .is_ok()
            {}
        }
        Message::SaveFile => {
            save_file(main_struct, sender);
        }
        // Edit
        Message::ClearEditor => {
            main_struct.buffer.set_text("");
            main_struct.buffer.undo();
        }
        // View
        Message::ToggleFileList => {
            main_struct
                .side_bar_box
                .set_visible(!main_struct.side_bar_box.is_visible());
            save_settings(main_struct);
        }
        Message::ToggleHiddenFiles => {
            main_struct.view_hidden = !main_struct.view_hidden;
            save_settings(main_struct);
        }
        Message::ToggleMiniMap => {
            main_struct
                .mini_map
                .set_visible(!main_struct.mini_map.is_visible());
            save_settings(main_struct);
        }
        Message::ToggleBufferStyleScheme => {
            toggle_buffer_style(main_struct);
        }
        Message::ToggleFullscreen => match main_struct.root.is_fullscreen() {
            true => {
                main_struct.root.set_fullscreened(false);
            }
            false => {
                main_struct.root.set_fullscreened(true);
            }
        },
        // About
        Message::ShowKeyboardShortcuts => {
            crate::util::dialogs::create_keyboard_shortcut_dialog();
        }
        Message::ShowPreferences => {
            crate::util::dialogs::create_preferences_dialog(main_struct, sender);
        }
        Message::ShowAbout => {
            crate::util::dialogs::create_about_dialog();
        }
        // File list
        Message::DeleteItem => {}
        Message::OpenFolderExternal => {
            if fs::exists(&main_struct.current_folder_path).is_ok()
                && open::that(&main_struct.current_folder_path).is_ok()
            {}
        }
        // Other
        Message::LoadSettings => {
            println!("Loading Settings...");
            load_settings(main_struct);
        }
        Message::UpdateMonospace(value) => {
            main_struct.editor.set_monospace(value);
            save_settings(main_struct);
        }
        Message::UpdateTabType(use_spaces) => {
            main_struct
                .editor
                .set_insert_spaces_instead_of_tabs(use_spaces);
            save_settings(main_struct);
        }
        Message::UpdateTabWidth(tab_width) => {
            main_struct.editor.set_tab_width(tab_width);
            save_settings(main_struct);
        }
        Message::UpdateVisibility(item, visibilty) => {
            match item {
                ItemVis::SideBar => {
                    main_struct.side_bar_box.set_visible(visibilty);
                }
                ItemVis::MiniMap => {
                    main_struct.mini_map.set_visible(visibilty);
                }
                ItemVis::HiddenFiles => {
                    main_struct.view_hidden = visibilty;
                }
            }
            save_settings(main_struct);
        }
        Message::CursorPositionChanged => {}
        Message::QuickToast(toast_text) => {
            main_struct.toast_overlay.add_toast(Toast::new(&toast_text))
        }
        Message::Ignore => {}
    }
}
