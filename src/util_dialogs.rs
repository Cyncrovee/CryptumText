use gtk4::{AboutDialog, glib::clone};
use libadwaita::{
    PreferencesDialog, PreferencesGroup, PreferencesPage, PreferencesRow, SpinRow, SwitchRow,
    prelude::*,
};
use sourceview5::prelude::ViewExt;

use crate::app_model::{MainStruct, Message};

pub fn create_preferences_dialog(
    main_struct: &mut MainStruct,
    sender: relm4::ComponentSender<MainStruct>,
) {
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

    let page = PreferencesPage::builder().title("Page").build();
    page.add(&tab_group);

    let dialog = PreferencesDialog::builder()
        .title("Preferences")
        .can_close(true)
        .child(&page)
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
