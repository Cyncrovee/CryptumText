use relm4::gtk::gio::{Menu, MenuItem, MenuModel};

pub fn menu_bar() -> MenuModel {
    let menu = Menu::new();

    // File
    let file_menu = Menu::new();
    let new_section = Menu::new();
    new_section.insert_item(0, &MenuItem::new(Some("New File"), Some("file.new_file")));
    file_menu.insert_section(0, None, &new_section);
    let load_section = Menu::new();
    load_section.insert_item(0, &MenuItem::new(Some("Load File"), Some("file.open")));
    load_section.insert_item(
        1,
        &MenuItem::new(Some("Load Folder"), Some("file.open_folder")),
    );
    file_menu.insert_section(1, None, &load_section);
    let save_section = Menu::new();
    save_section.insert_item(0, &MenuItem::new(Some("Save"), Some("file.save")));
    save_section.insert_item(1, &MenuItem::new(Some("Save As..."), Some("file.save_as")));
    file_menu.insert_section(2, None, &save_section);

    // Edit
    let edit_menu = Menu::new();
    let undo_redo_section = Menu::new();
    undo_redo_section.insert_item(0, &MenuItem::new(Some("Undo"), Some("edit.undo")));
    undo_redo_section.insert_item(1, &MenuItem::new(Some("Redo"), Some("edit.redo")));
    let clipboard_section = Menu::new();
    clipboard_section.insert_item(0, &MenuItem::new(Some("Cut"), Some("edit.cut")));
    clipboard_section.insert_item(1, &MenuItem::new(Some("Copy"), Some("edit.copy")));
    clipboard_section.insert_item(2, &MenuItem::new(Some("Paste"), Some("edit.paste")));
    edit_menu.insert_section(0, None, &undo_redo_section);
    edit_menu.insert_section(1, None, &clipboard_section);
    let other_section = Menu::new();
    other_section.insert_item(1, &MenuItem::new(Some("Clear"), Some("edit.clear")));
    edit_menu.insert_section(2, None, &other_section);

    // View
    let view_menu = Menu::new();
    let toggle_section = Menu::new();
    toggle_section.insert_item(
        0,
        &MenuItem::new(
            Some("Toggle File List Visibilty"),
            Some("view.toggle_file_list"),
        ),
    );
    toggle_section.insert_item(
        1,
        &MenuItem::new(
            Some("Toggle Mini Map Visibilty"),
            Some("view.toggle_mini_map"),
        ),
    );
    toggle_section.insert_item(
        2,
        &MenuItem::new(
            Some("Toggle Editor Theme (Light/Dark)"),
            Some("view.toggle_buffer_style_scheme"),
        ),
    );
    view_menu.insert_section(0, None, &toggle_section);

    menu.insert_submenu(0, Some("File"), &file_menu);
    menu.insert_submenu(1, Some("Edit"), &edit_menu);
    menu.insert_submenu(2, Some("View"), &view_menu);

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
