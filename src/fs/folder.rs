use gtk4::{
    DirectoryList, Label, SignalListItemFactory, SingleSelection, TreeExpander, TreeListModel,
    TreeListRow,
    gio::{File, FileInfo, FileType},
    glib::clone,
    prelude::ListItemExt,
};
use sourceview5::prelude::*;

use crate::app::model::{Msg, State};

pub fn load_folder_view(state: &mut State, sender: relm4::ComponentSender<State>) {
    let dir_list = DirectoryList::new(
        Some("standard::*"),
        Some(&File::for_path(&state.current_folder_path)),
    );
    let model = TreeListModel::new(dir_list, false, false, move |o| {
        if let Some(file_info) = o.downcast_ref::<FileInfo>()
            && file_info.file_type() == FileType::Directory
            && let Some(file) = file_info
                .attribute_object("standard::file")
                .and_dynamic_cast_ref::<File>()
        {
            let dir_list_local = DirectoryList::new(Some("standard::*"), Some(file));
            Some(dir_list_local.into())
        } else {
            None
        }
    });
    let selection = SingleSelection::new(Some(model.clone()));
    selection.connect_selection_changed(clone!(
        #[strong]
        sender,
        move |selection, _, _| {
            if let Some(row) = selection.selected_item().and_downcast::<TreeListRow>()
                && let Some(file_info) = row.item().and_downcast::<FileInfo>()
            {
                sender.input(Msg::LoadFileFromTree(file_info));
            }
        }
    ));
    let factory = SignalListItemFactory::new();
    factory.connect_setup(move |_, list_item| {
        list_item.set_child(Some(
            &TreeExpander::builder().child(&Label::new(None)).build(),
        ));
    });
    factory.connect_bind(move |_, list_item| {
        if let Some(row) = list_item.item().and_downcast::<TreeListRow>()
            && let Some(file_info) = row.item().and_downcast::<FileInfo>()
            && let Some(tree) = list_item.child().and_downcast::<TreeExpander>()
            && let Some(label) = tree.child().and_downcast::<Label>()
        {
            if file_info.file_type() == FileType::Directory {
                label.set_text(&format!("{}/", &file_info.display_name()));
            } else {
                label.set_text(&file_info.display_name());
            }
            tree.set_list_row(Some(&row));
        }
    });
    state.file_view.set_model(Some(&selection));
    state.file_view.set_factory(Some(&factory));
}
