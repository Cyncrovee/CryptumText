use gtk4::{
    DirectoryList, Label, SignalListItemFactory, SingleSelection, TreeExpander, TreeListModel,
    gio::{File, FileInfo},
    prelude::ListItemExt,
};
use sourceview5::prelude::Cast;

use crate::app::model::MainStruct;

pub fn load_folder_view(main_struct: &mut MainStruct) {
    let file = File::for_path(&main_struct.current_folder_path);
    let dir_list = DirectoryList::new(Some("standard::name"), Some(&file));
    let model = TreeListModel::new(dir_list, false, false, |o| {
        // let root_dir = Path::new(main_struct.current_folder_path);
        // let name = o.downcast_ref::<FileInfo>().unwrap().name();
        // let mut root_dir_full = root_dir.to_path_buf();
        // root_dir_full.push(name);
        // if root_dir_full.is_dir() {
        //     let dir_list_local = DirectoryList::new(
        //         Some("standard::name"),
        //         Some(&File::for_path(root_dir_full.as_path())),
        //     );
        //     return Some(dir_list_local.into());
        // }
        None
    });
    let selection = SingleSelection::new(Some(model.model()));
    let factory = SignalListItemFactory::new();
    factory.connect_setup(move |_, list_item| {
        list_item.set_child(Some(&TreeExpander::new()));
    });
    factory.connect_bind(move |_, list_item| {
        list_item.set_child(Some(
            &TreeExpander::builder()
                .child(&Label::new(Some(
                    list_item
                        .item()
                        .unwrap()
                        .downcast_ref::<FileInfo>()
                        .unwrap()
                        .name()
                        .to_str()
                        .unwrap(),
                )))
                .build(),
        ));
    });
    main_struct.file_view.set_model(Some(&selection));
    main_struct.file_view.set_factory(Some(&factory));
}
