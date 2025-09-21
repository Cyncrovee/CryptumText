use relm4::gtk::gio::{Menu, MenuItem, MenuModel};

pub fn menu_bar() -> MenuModel {
    let menu = Menu::new();

    menu.insert_item(0, &MenuItem::new(Some("New File"), Some("file.new_file")));

    // Load
    let load_section = Menu::new();
    load_section.insert_item(0, &MenuItem::new(Some("Load File"), Some("file.open")));
    load_section.insert_item(
        1,
        &MenuItem::new(Some("Load Folder"), Some("file.open_folder")),
    );
    menu.insert_section(1, None, &load_section);

    // Save
    let save_section = Menu::new();
    save_section.insert_item(0, &MenuItem::new(Some("Save"), Some("file.save")));
    save_section.insert_item(1, &MenuItem::new(Some("Save As..."), Some("file.save_as")));
    menu.insert_section(2, None, &save_section);

    // Edit
    let edit_section = Menu::new();
    edit_section.insert_item(0, &MenuItem::new(Some("Clear"), Some("edit.clear")));
    menu.insert_section(3, None, &edit_section);

    // Toggle
    let toggle_section = Menu::new();
    toggle_section.insert_item(
        0,
        &MenuItem::new(
            Some("Toggle Editor Theme (Light/Dark)"),
            Some("view.toggle_buffer_style_scheme"),
        ),
    );
    toggle_section.insert_item(
        1,
        &MenuItem::new(Some("Toggle Fullscreen"), Some("view.toggle_fullscreen")),
    );
    menu.insert_section(4, None, &toggle_section);

    let extras_section = Menu::new();
    extras_section.insert_item(
        0,
        &MenuItem::new(Some("Preferences"), Some("about.show_preferences")),
    );
    extras_section.insert_item(
        1,
        &MenuItem::new(
            Some("Keyboard Shortcuts"),
            Some("about.show_keyboard_shortcuts"),
        ),
    );
    extras_section.insert_item(
        2,
        &MenuItem::new(Some("About Cryptum Text"), Some("about.show_about")),
    );
    menu.insert_section(5, None, &extras_section);

    menu.into()
}

pub fn file_list_context_menu_model() -> MenuModel {
    let menu = Menu::new();

    menu.insert_item(0, &MenuItem::new(Some("New File"), Some("file.new_file")));
    menu.insert_item(
        1,
        &MenuItem::new(
            Some("Open Folder in File Browser"),
            Some("list.open_folder_external"),
        ),
    );

    menu.into()
}

pub fn file_list_context_menu_model_item() -> MenuModel {
    let menu = Menu::new();

    menu.insert_item(0, &MenuItem::new(Some("New File"), Some("file.new_file")));
    menu.insert_item(
        1,
        &MenuItem::new(Some("Move to Trash"), Some("list.delete_item")),
    );
    menu.insert_item(
        2,
        &MenuItem::new(
            Some("Open Folder in File Browser"),
            Some("list.open_folder_external"),
        ),
    );

    menu.into()
}
