use std::path::PathBuf;

use libadwaita::{ToastOverlay, WindowTitle};
use relm4::{Controller, prelude::*};
use relm4_components::{open_dialog::OpenDialog, save_dialog::SaveDialog};
use serde::{Deserialize, Serialize};
use sourceview5::LanguageManager;

// Structs
pub struct MainStruct {
    // Containers
    pub root: libadwaita::ApplicationWindow,
    pub side_bar_box: gtk::Box,
    // Widgets
    pub file_list: gtk::ListBox,
    pub file_list_context_menu: gtk::PopoverMenu,
    pub editor: sourceview5::View,
    pub buffer: sourceview5::Buffer,
    pub language_manager: LanguageManager,
    pub open_dialog: Controller<OpenDialog>,
    pub folder_dialog: Controller<OpenDialog>,
    pub save_as_dialog: Controller<SaveDialog>,
    pub title: WindowTitle,
    pub file_type_label: gtk::Label,
    pub cursor_position_label: gtk::Label,
    pub mini_map: sourceview5::Map,
    pub toast_overlay: ToastOverlay,
    // Misc
    pub current_file_path: String,
    pub current_folder_path: String,
    pub buffer_style: Option<sourceview5::StyleScheme>,
    pub view_hidden: bool,
    pub git_info: (String, bool),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AppSettings {
    pub editor_theme: String,
    pub editor_monospace: bool,
    pub editor_use_spaces_for_tabs: bool,
    pub editor_tab_width: u32,
    pub view_mini_map: bool,
    pub view_file_list: bool,
    pub view_hidden_files: bool,
}

pub struct WidgetStruct {}

// Enums
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
    ClearEditor,
    // View
    ToggleFileList,
    ToggleHiddenFiles,
    ToggleMiniMap,
    ToggleBufferStyleScheme,
    ToggleFullscreen,
    // About
    ShowKeyboardShortcuts,
    ShowPreferences,
    ShowAbout,
    // File list
    FileListContext(i32, i32),
    DeleteItem,
    OpenFolderExternal,
    // Other
    LoadSettings,
    UpdateMonospace(bool),
    UpdateTabType(bool),
    UpdateTabWidth(u32),
    UpdateVisibility(ItemVis, bool),
    UpDir,
    RefreshFileList,
    CursorPositionChanged,
    Ignore,
}

#[derive(Debug)]
pub enum ItemVis {
    SideBar,
    MiniMap,
    HiddenFiles,
}
