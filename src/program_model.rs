use std::path::PathBuf;

use libadwaita::WindowTitle;
use relm4::{Controller, prelude::*};
use relm4_components::{open_dialog::OpenDialog, save_dialog::SaveDialog};
use serde::{Deserialize, Serialize};
use sourceview5::LanguageManager;

pub struct MainStruct {
    // Non-widgets
    pub current_file_path: String,
    pub current_folder_path: String,
    pub clipboard: gtk::gdk::Clipboard,
    pub buffer_style: Option<sourceview5::StyleScheme>,
    pub current_style: String,
    // Widgets
    pub file_list: gtk::ListBox,
    pub buffer: sourceview5::Buffer,
    pub language_manager: LanguageManager,
    pub open_dialog: Controller<OpenDialog>,
    pub folder_dialog: Controller<OpenDialog>,
    pub save_as_dialog: Controller<SaveDialog>,
    pub title: WindowTitle,
    pub file_type_label: gtk::Label,
    pub cursor_position_label: gtk::Label,
    pub mini_map: sourceview5::Map,
}

pub struct WidgetStruct {}

#[derive(Debug)]
pub enum Message {
    // File
    NewFile,
    LoadFileFromList,
    FolderRequest,
    FolderResponse(PathBuf),
    OpenRequest,
    OpenResponse(PathBuf),
    SaveAsRequest,
    SaveAsResponse(PathBuf),
    SaveFile,
    // Edit
    Undo,
    Redo,
    CutEditor,
    CopyEditor,
    PasteEditor,
    ClearEditor,
    // View
    ToggleFileList,
    ToggleMiniMap,
    ToggleBufferStyleScheme,
    // About
    ShowAbout,
    // Other
    CursorPostitionChanged,
    Ignore,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AppSettings {
    pub view_mini_map: bool,
    pub view_file_list: bool,
}
