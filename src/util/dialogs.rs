use gtk4::{AboutDialog, glib::clone};
use libadwaita::{
    HeaderBar, PreferencesDialog, PreferencesGroup, PreferencesPage, PreferencesRow, SpinRow,
    SwitchRow, ToolbarView, WindowTitle, prelude::*,
};
use sourceview5::prelude::ViewExt;

use crate::app::model::{MainStruct, Message, ItemVis};

pub fn create_preferences_dialog(
    main_struct: &mut MainStruct,
    sender: relm4::ComponentSender<MainStruct>,
) {
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
        move |row| sender.input(Message::UpdateVisibility(
            ItemVis::SideBar,
            row.is_active()
        ))
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
        move |row| sender.input(Message::UpdateVisibility(
            ItemVis::MiniMap,
            row.is_active()
        ))
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

pub fn create_about_dialog() {
    AboutDialog::builder()
        .program_name("Cryptum Text")
        .version("Dev Version")
        .copyright("Ella Hart (Cyncrovee)")
        .license_type(gtk4::License::Gpl30Only)
        .build()
        .show();
}
