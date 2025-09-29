use std::path::PathBuf;

use gtk4::{
    DirectoryList, Label, SignalListItemFactory, SingleSelection, TreeExpander, TreeListModel,
    gio::{File, FileInfo},
    prelude::ListItemExt,
};
use sourceview5::prelude::*;

use crate::app::model::State;

pub fn load_folder_view(main_struct: &mut State) {
    let path = main_struct.current_folder_path.clone();
    let file = File::for_path(&path);
    let dir_list = DirectoryList::new(Some("standard::name"), Some(&file));
    let model = TreeListModel::new(dir_list, false, false, move |o| {
        let dir_str = o.downcast_ref::<FileInfo>().unwrap().name();
        let mut dir_path = PathBuf::new();
        dir_path.push(&path);
        dir_path.push(&dir_str);
        if dir_path.is_dir() {
            let dir_list_local = DirectoryList::new(None, Some(&File::for_path(dir_path)));
            Some(dir_list_local.into())
        } else {
            None
        }
    });
    let selection = SingleSelection::new(Some(model.model()));
    let factory = SignalListItemFactory::new();
    factory.connect_setup(move |_, list_item| {
        list_item.set_child(Some(
            &TreeExpander::builder().child(&Label::new(None)).build(),
        ));
    });
    factory.connect_bind(move |_, list_item| {
        let item = list_item.item().unwrap();
        let file_info = item.downcast_ref::<FileInfo>().unwrap();
        let tree = list_item.child().and_downcast::<TreeExpander>().unwrap();
        tree.set_list_row(model.row(list_item.position()).as_ref());
        tree.set_child(Some(&Label::new(Some(file_info.name().to_str().unwrap()))));
    });
    main_struct.file_view.set_model(Some(&selection));
    main_struct.file_view.set_factory(Some(&factory));
}
