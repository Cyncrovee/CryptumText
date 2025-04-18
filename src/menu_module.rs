use relm4::gtk::gio::{Menu, MenuItem, MenuModel};

pub fn menu_bar() -> MenuModel {
    let menu = Menu::new();

    // File
    let file_menu = Menu::new();
    let new_section = Menu::new();
    new_section.insert_item(0, &MenuItem::new(Some("New File"), Some("win.new_file")));
    file_menu.insert_section(0, None, &new_section);
    let load_section = Menu::new();
    load_section.insert_item(0, &MenuItem::new(Some("Load File"), Some("win.open")));
    load_section.insert_item(
        1,
        &MenuItem::new(Some("Load Folder"), Some("win.open_folder")),
    );
    file_menu.insert_section(1, None, &load_section);
    let save_section = Menu::new();
    save_section.insert_item(0, &MenuItem::new(Some("Save"), Some("win.save")));
    save_section.insert_item(1, &MenuItem::new(Some("Save As..."), Some("win.save_as")));
    file_menu.insert_section(2, None, &save_section);

    // Edit
    let edit_menu = Menu::new();
    let clipboard_section = Menu::new();
    clipboard_section.insert_item(0, &MenuItem::new(Some("Cut"), Some("win.cut")));
    clipboard_section.insert_item(1, &MenuItem::new(Some("Copy"), Some("win.copy")));
    clipboard_section.insert_item(2, &MenuItem::new(Some("Paste"), Some("win.paste")));
    edit_menu.insert_section(0, None, &clipboard_section);
    let other_section = Menu::new();
    other_section.insert_item(1, &MenuItem::new(Some("Clear"), Some("win.clear")));
    edit_menu.insert_section(1, None, &other_section);

    menu.insert_submenu(0, Some("File"), &file_menu);
    menu.insert_submenu(1, Some("Edit"), &edit_menu);

    return menu.into();
}
