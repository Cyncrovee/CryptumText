use std::path::Path;

use gtk4::{
    DirectoryList, Label, SignalListItemFactory, SingleSelection, TreeExpander, TreeListModel,
    gio::{File, FileInfo},
    prelude::ListItemExt,
};
use sourceview5::prelude::{Cast, CastNone};

use crate::app::model::MainStruct;

pub fn load_folder_view(main_struct: &mut MainStruct) {
    let file = File::for_path(&main_struct.current_folder_path);
    let dir_list = DirectoryList::new(Some("standard::name,standard::path"), Some(&file));
    let model = TreeListModel::new(dir_list, true, true, |o| {
        let dir_str = o
            .downcast_ref::<FileInfo>()
            .unwrap()
            .attribute_string("path")
            .unwrap()
            .to_string();
        let dir_path = Path::new(&dir_str);
        if dir_path.is_dir() {
            let dir_list_local = DirectoryList::new(
                Some("standard::name,standard::path"),
                Some(&File::for_path(dir_path)),
            );
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
        tree.set_child(Some(&Label::new(Some(file_info.name().to_str().unwrap()))));
    });
    main_struct.file_view.set_model(Some(&selection));
    main_struct.file_view.set_factory(Some(&factory));
}
