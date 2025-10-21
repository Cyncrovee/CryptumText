use gtk4::{
    DirectoryList, Label, SignalListItemFactory, SingleSelection, TreeExpander, TreeListModel,
    TreeListRow,
    gio::{File, FileInfo, FileType},
    prelude::ListItemExt,
};
use sourceview5::prelude::*;

use crate::app::model::State;

pub fn load_folder_view(state: &mut State) {
    let dir_list = DirectoryList::new(
        Some("standard::*"),
        Some(&File::for_path(&state.current_folder_path)),
    );
    let model = TreeListModel::new(dir_list, false, false, move |o| {
        let file_info = o.downcast_ref::<FileInfo>().unwrap();
        if file_info.file_type() == FileType::Directory {
            let dir_list_local = DirectoryList::new(
                Some("standard::*"),
                Some(
                    file_info
                        .attribute_object("standard::file")
                        .unwrap()
                        .dynamic_cast_ref::<File>()
                        .unwrap(),
                ),
            );
            Some(dir_list_local.into())
        } else {
            None
        }
    });
    let selection = SingleSelection::new(Some(model.clone()));
    let factory = SignalListItemFactory::new();
    factory.connect_setup(move |_, list_item| {
        list_item.set_child(Some(
            &TreeExpander::builder().child(&Label::new(None)).build(),
        ));
    });
    factory.connect_bind(move |_, list_item| {
        let row = list_item.item().and_downcast::<TreeListRow>().unwrap();
        let file_info = row.item().and_downcast::<FileInfo>().unwrap();
        let tree = list_item.child().and_downcast::<TreeExpander>().unwrap();
        let label = tree.child().and_downcast::<Label>().unwrap();
        if file_info.file_type() == FileType::Directory {
            label.set_text(&format!("{}/", &file_info.display_name()));
        } else {
            label.set_text(&file_info.display_name());
        }
        tree.set_list_row(Some(&row));
    });
    state.file_view.set_model(Some(&selection));
    state.file_view.set_factory(Some(&factory));
}
