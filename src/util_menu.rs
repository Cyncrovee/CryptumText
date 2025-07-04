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
            Some("Toggle File List Visibilty"),
            Some("view.toggle_file_list"),
        ),
    );
    #[cfg(unix)]
    {
        toggle_section.insert_item(
            1,
            &MenuItem::new(
                Some("Toggle Hidden Files"),
                Some("view.toggle_hidden_files"),
            ),
        );
    }
    toggle_section.insert_item(
        2,
        &MenuItem::new(
            Some("Toggle Mini Map Visibilty"),
            Some("view.toggle_mini_map"),
        ),
    );
    toggle_section.insert_item(
        3,
        &MenuItem::new(
            Some("Toggle Editor Theme (Light/Dark)"),
            Some("view.toggle_buffer_style_scheme"),
        ),
    );
    menu.insert_section(4, None, &toggle_section);

    return menu.into();
}

pub fn extras_menu_bar() -> MenuModel {
    let menu = Menu::new();

    menu.insert_item(
        0,
        &MenuItem::new(Some("About Cryptum Text"), Some("about.show_about")),
    );

    return menu.into();
}

pub fn file_list_context_menu_model() -> MenuModel {
    let menu = Menu::new();

    menu.insert_item(0, &MenuItem::new(Some("New File"), Some("file.new_file")));

    return menu.into();
}

pub fn file_list_context_menu_model_item() -> MenuModel {
    let menu = Menu::new();

    menu.insert_item(0, &MenuItem::new(Some("New File"), Some("file.new_file")));
    menu.insert_item(
        1,
        &MenuItem::new(Some("Move to Trash"), Some("list.delete_item")),
    );

    return menu.into();
}
