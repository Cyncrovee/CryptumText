use gtk4::{
    AboutDialog, ShortcutsGroup, ShortcutsSection, ShortcutsShortcut, ShortcutsWindow, glib::clone,
};
use libadwaita::{
    HeaderBar, PreferencesDialog, PreferencesGroup, PreferencesPage, PreferencesRow, SpinRow,
    SwitchRow, ToolbarView, WindowTitle, prelude::*,
};
use sourceview5::prelude::ViewExt;

use crate::app::model::{ItemVis, MainStruct, Message};

pub fn create_preferences_dialog(
    main_struct: &mut MainStruct,
    sender: relm4::ComponentSender<MainStruct>,
) {
    // Editor group setup
    let is_monospace_switch_row = SwitchRow::builder()
        .title("Monospace")
        .activatable(false)
        .active(main_struct.editor.is_monospace())
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
        .active(main_struct.editor.is_insert_spaces_instead_of_tabs())
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
        .build();
    let tabs_amount = main_struct.editor.tab_width();
    let tabs_amount = tabs_amount as f64;
    tab_spaces_spin_row.set_adjustment(Some(&gtk4::Adjustment::new(
        tabs_amount,
        1.0,
        32.0,
        1.0,
        5.0,
        0.0,
    )));
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
        .active(main_struct.side_bar_box.is_visible())
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
        .active(main_struct.mini_map.is_visible())
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
        .active(main_struct.view_hidden)
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
    let title = WindowTitle::new("Preferences", "");
    let header = HeaderBar::builder().title_widget(&title).build();
    let toolbar = ToolbarView::builder().build();
    toolbar.add_top_bar(&header);
    toolbar.set_content(Some(&page));

    let dialog = PreferencesDialog::builder()
        .title("Preferences")
        .can_close(true)
        .child(&toolbar)
        .build();
    dialog.present(Some(&main_struct.root));
}

pub fn create_keyboard_shortcut_dialog() {
    // File shortcut group
    let new_file_shortcut = ShortcutsShortcut::builder()
        .title("New File")
        .accelerator("<control><shift>n")
        .build();
    let open_file_shortcut = ShortcutsShortcut::builder()
        .title("Open File")
        .accelerator("<control>o")
        .build();
    let open_folder_shortcut = ShortcutsShortcut::builder()
        .title("Open Folder")
        .accelerator("<control><shift>o")
        .build();
    let save_file_shortcut = ShortcutsShortcut::builder()
        .title("Save File")
        .accelerator("<control>s")
        .build();
    let save_file_as_shortcut = ShortcutsShortcut::builder()
        .title("Save File As")
        .accelerator("<control><shift>s")
        .build();
    let file_group = ShortcutsGroup::builder().title("File").build();
    file_group.append(&new_file_shortcut);
    file_group.append(&open_file_shortcut);
    file_group.append(&open_folder_shortcut);
    file_group.append(&save_file_shortcut);
    file_group.append(&save_file_as_shortcut);

    // View shortcut group
    let toggle_list_shortcut = ShortcutsShortcut::builder()
        .title("Toggle File List Visibility")
        .accelerator("<control><alt>f")
        .build();
    let toggle_mini_map_shortcut = ShortcutsShortcut::builder()
        .title("Toggle Mini Map Visibility")
        .accelerator("<control><alt>m")
        .build();
    let toggle_hidden_shortcut = ShortcutsShortcut::builder()
        .title("Toggle Hidden Files Visibility (UNIX)")
        .accelerator("<control>h")
        .build();
    let toggle_fullscreen_shortcut = ShortcutsShortcut::builder()
        .title("Toggle Fullscreen")
        .accelerator("F11")
        .build();
    let edit_group = ShortcutsGroup::builder().title("View").build();
    edit_group.append(&toggle_list_shortcut);
    edit_group.append(&toggle_mini_map_shortcut);
    edit_group.append(&toggle_hidden_shortcut);
    edit_group.append(&toggle_fullscreen_shortcut);

    // About shortcut group
    let show_preferences_shortcut = ShortcutsShortcut::builder()
        .title("Show Preferences Dialog")
        .accelerator("<control>comma")
        .build();
    let show_keyboard_shortcuts_shortcut = ShortcutsShortcut::builder()
        .title("Show Keyboard Shortcuts Dialog")
        .accelerator("<control>question")
        .build();
    let about_group = ShortcutsGroup::builder().title("About").build();
    about_group.append(&show_preferences_shortcut);
    about_group.append(&show_keyboard_shortcuts_shortcut);

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
