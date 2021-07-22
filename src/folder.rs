use gtk::{Builder, CellRendererText, ListStore, TreeView, TreeViewColumn, prelude::{BuilderExtManual, CellLayoutExt, GtkListStoreExtManual, StaticType, ToValue, TreeViewColumnExt, TreeViewExt}};

pub struct Folder {
    id: String,
    treeview: TreeView

}

impl Folder {
    pub fn new(builder: &Builder, id: &str) -> Self {
        let treeview : TreeView = builder.object(&(id.to_string() + "-folder").to_string()).unwrap();

        append_column(&treeview, 0);
        append_column(&treeview, 1);
        append_column(&treeview, 2);
        let model = create_and_fill_model();
        treeview.set_model(Some(&model));

        Self {
            id: id.to_string(),
            treeview: treeview
        }
    }
}

fn append_column(tree: &TreeView, id: i32) {
    let column = TreeViewColumn::new();
    let cell = CellRendererText::new();

    column.set_title(&format!("Spalte {}", id));
    column.set_resizable(true);
    column.set_expand(true);
    column.pack_start(&cell, true);
    // Association of the view's column with the model's `id` column.
    column.add_attribute(&cell, "text", id);
    tree.append_column(&column);
}

fn create_and_fill_model() -> ListStore {
    // Creation of a model with two rows.
    let model = ListStore::new(&[u32::static_type(), String::static_type()]);
    
    // Filling up the tree view.
    let entries = &["Michel", "Sara", "Liam", "Zelda", "Neo", "Octopus master"];
    for (i, entry) in entries.iter().enumerate() {
        model.insert_with_values(None, &[(0, &(i as u32 + 1).to_value()), (1, &entry.to_string().to_value())]);
    }
    model
}



