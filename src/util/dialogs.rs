use gtk4::{
    AboutDialog, ShortcutsGroup, ShortcutsSection, ShortcutsShortcut, ShortcutsWindow, glib::clone,
};
use libadwaita::{
    HeaderBar, PreferencesDialog, PreferencesGroup, PreferencesPage, PreferencesRow, SpinRow,
    SwitchRow, ToolbarView, WindowTitle, prelude::*,
};
use sourceview5::prelude::ViewExt;

use crate::app::model::{ItemVis, Message, State};

pub fn create_preferences_dialog(state: &mut State, sender: relm4::ComponentSender<State>) {
    // Editor group setup
    let is_monospace_switch_row = SwitchRow::builder()
        .title("Monospace")
        .activatable(false)
        .active(state.editor.is_monospace())
        .build();
    is_monospace_switch_row.connect_active_notify(clone!(
        #[strong]
        sender,
        move |row| sender.input(Message::UpdateMonospace(row.is_active()))
    ));
    let editor_group = PreferencesGroup::builder().title("Editor").build();
    editor_group.add(
        &PreferencesRow::builder()
            .title("Use Monospace Font")
            .activatable(false)
            .child(&is_monospace_switch_row)
            .height_request(60)
            .build(),
    );

    // Tab group setup
    let tab_type_switch_row = SwitchRow::builder()
        .title("Enable Using Spaces for Tabs")
        .activatable(false)
        .active(state.editor.is_insert_spaces_instead_of_tabs())
        .build();
    tab_type_switch_row.connect_active_notify(clone!(
        #[strong]
        sender,
        move |row| sender.input(Message::UpdateTabType(row.is_active()))
    ));
    let tab_spaces_spin_row = SpinRow::builder()
        .title("Number of Spaces to Use for Tabs")
        .activatable(false)
        .climb_rate(1.0)
        .digits(0)
        .adjustment(&gtk4::Adjustment::new(
            state.editor.tab_width() as f64,
            1.0,
            32.0,
            1.0,
            5.0,
            0.0,
        ))
        .build();
    tab_spaces_spin_row.connect_value_notify(clone!(
        #[strong]
        sender,
        move |row| sender.input(Message::UpdateTabWidth(row.value() as u32))
    ));
    let tab_group = PreferencesGroup::builder().title("Tabs").build();
    tab_group.add(
        &PreferencesRow::builder()
            .title("Set Tab Size")
            .activatable(false)
            .child(&tab_type_switch_row)
            .height_request(60)
            .build(),
    );
    tab_group.add(
        &PreferencesRow::builder()
            .title("Set Tab Size")
            .activatable(false)
            .child(&tab_spaces_spin_row)
            .height_request(60)
            .build(),
    );

    // Visibility group setup
    let file_list_visibilty_spin_row = SwitchRow::builder()
        .title("Side Bar Visibility")
        .subtitle("Ctrl+Alt+F")
        .activatable(false)
        .active(state.side_bar_box.is_visible())
        .build();
    file_list_visibilty_spin_row.connect_active_notify(clone!(
        #[strong]
        sender,
        move |row| sender.input(Message::UpdateVisibility(ItemVis::SideBar, row.is_active()))
    ));
    let mini_map_visibilty_spin_row = SwitchRow::builder()
        .title("Mini Map Visibility")
        .subtitle("Ctrl+Alt+M")
        .activatable(false)
        .active(state.mini_map.is_visible())
        .build();
    mini_map_visibilty_spin_row.connect_active_notify(clone!(
        #[strong]
        sender,
        move |row| sender.input(Message::UpdateVisibility(ItemVis::MiniMap, row.is_active()))
    ));
    let hidden_files_visibilty_spin_row = SwitchRow::builder()
        .title("Hidden Files Visibility")
        .subtitle("Ctrl+H")
        .activatable(false)
        .active(state.view_hidden)
        .build();
    hidden_files_visibilty_spin_row.connect_active_notify(clone!(
        #[strong]
        sender,
        move |row| sender.input(Message::UpdateVisibility(
            ItemVis::HiddenFiles,
            row.is_active()
        ))
    ));
    let visibility_group = PreferencesGroup::builder().title("Visibility").build();
    visibility_group.add(
        &PreferencesRow::builder()
            .title("Side Bar Visibility")
            .activatable(false)
            .child(&file_list_visibilty_spin_row)
            .height_request(60)
            .build(),
    );
    visibility_group.add(
        &PreferencesRow::builder()
            .title("Mini Map Visibility")
            .activatable(false)
            .child(&mini_map_visibilty_spin_row)
            .height_request(60)
            .build(),
    );
    #[cfg(unix)]
    visibility_group.add(
        &PreferencesRow::builder()
            .title("Hidden Files Visibility")
            .activatable(false)
            .child(&hidden_files_visibilty_spin_row)
            .height_request(60)
            .build(),
    );

    // Page and dialog setup
    let page = PreferencesPage::builder().title("Page").build();
    page.add(&editor_group);
    page.add(&tab_group);
    page.add(&visibility_group);
    let toolbar = ToolbarView::builder().build();
    toolbar.add_top_bar(
        &HeaderBar::builder()
            .title_widget(&WindowTitle::new("Preferences", ""))
            .build(),
    );
    toolbar.set_content(Some(&page));

    let dialog = PreferencesDialog::builder()
        .title("Preferences")
        .can_close(true)
        .child(&toolbar)
        .build();
    dialog.present(Some(&state.root));
}

pub fn create_keyboard_shortcut_dialog() {
    // File shortcut group
    let file_shortcut_array: [ShortcutsShortcut; 5] = [
        ShortcutsShortcut::builder()
            .title("New File")
            .accelerator("<control><shift>n")
            .build(),
        ShortcutsShortcut::builder()
            .title("Open File")
            .accelerator("<control>o")
            .build(),
        ShortcutsShortcut::builder()
            .title("Open Folder")
            .accelerator("<control><shift>o")
            .build(),
        ShortcutsShortcut::builder()
            .title("Save File")
            .accelerator("<control>s")
            .build(),
        ShortcutsShortcut::builder()
            .title("Save File As")
            .accelerator("<control><shift>s")
            .build(),
    ];
    let file_group = ShortcutsGroup::builder().title("File").build();
    for shortcut in file_shortcut_array {
        file_group.append(&shortcut);
    }

    // View shortcut group
    let view_shortcut_array: [ShortcutsShortcut; 4] = [
        ShortcutsShortcut::builder()
            .title("Toggle File List Visibility")
            .accelerator("<control><alt>f")
            .build(),
        ShortcutsShortcut::builder()
            .title("Toggle Mini Map Visibility")
            .accelerator("<control><alt>m")
            .build(),
        ShortcutsShortcut::builder()
            .title("Toggle Hidden Files Visibility (UNIX)")
            .accelerator("<control>h")
            .build(),
        ShortcutsShortcut::builder()
            .title("Toggle Fullscreen")
            .accelerator("F11")
            .build(),
    ];
    let edit_group = ShortcutsGroup::builder().title("View").build();
    for shortcut in view_shortcut_array {
        edit_group.append(&shortcut);
    }

    // About shortcut group
    let about_shortcuts_array: [ShortcutsShortcut; 2] = [
        ShortcutsShortcut::builder()
            .title("Show Preferences Dialog")
            .accelerator("<control>comma")
            .build(),
        ShortcutsShortcut::builder()
            .title("Show Keyboard Shortcuts Dialog")
            .accelerator("<control>question")
            .build(),
    ];
    let about_group = ShortcutsGroup::builder().title("About").build();
    for shortcut in about_shortcuts_array {
        about_group.append(&shortcut);
    }

    let section = ShortcutsSection::builder().build();
    section.append(&file_group);
    section.append(&edit_group);
    section.append(&about_group);
    ShortcutsWindow::builder().child(&section).build().show();
}

pub fn create_about_dialog() {
    AboutDialog::builder()
        .program_name("Cryptum Text")
        .version("Dev Version")
        .copyright("Ella Hart (Cyncrovee)")
        .license_type(gtk4::License::Gpl30Only)
        .build()
        .show();
}
