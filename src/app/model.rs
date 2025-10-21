use std::path::PathBuf;

use gtk4::gio::FileInfo;
use libadwaita::{ToastOverlay, WindowTitle};
use relm4::{Controller, prelude::*};
use relm4_components::{open_dialog::OpenDialog, save_dialog::SaveDialog};
use serde::{Deserialize, Serialize};
use sourceview5::LanguageManager;

// Structs
#[derive(Debug)]
pub struct State {
    // Containers
    pub root: libadwaita::ApplicationWindow,
    pub side_bar_box: gtk::Box,
    // Widgets
    pub file_view: gtk::ListView,
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
    pub current_file_path: PathBuf,
    pub current_folder_path: PathBuf,
    pub buffer_style: Option<sourceview5::StyleScheme>,
    pub view_hidden: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AppSettings {
    pub editor_theme: String,
    pub editor_monospace: bool,
    pub editor_use_spaces_for_tabs: bool,
    pub editor_tab_width: u32,
    pub view_mini_map: bool,
    pub view_hidden_files: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        AppSettings {
            editor_theme: "Adwaita".to_string(),
            editor_monospace: true,
            editor_use_spaces_for_tabs: true,
            editor_tab_width: 4,
            view_mini_map: true,
            view_hidden_files: false,
        }
    }
}

pub struct WidgetStruct {}

// Enums
#[derive(Debug)]
pub enum Message {
    // File
    NewFile,
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
    // File tree
    LoadFileFromTree(FileInfo),
    // Other
    LoadSettings,
    UpdateMonospace(bool),
    UpdateTabType(bool),
    UpdateTabWidth(u32),
    UpdateVisibility(ItemVis, bool),
    CursorPositionChanged,
    QuickToast(String),
    Ignore,
}

#[derive(Debug)]
pub enum ItemVis {
    SideBar,
    MiniMap,
    HiddenFiles,
}
