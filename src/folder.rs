use std::{cell::RefCell, rc::Rc};

use gtk::{Builder, CellRendererText, Entry, ListStore, TreeView, TreeViewColumn, gdk::ModifierType, prelude::*};

#[derive(Clone)]
pub struct Folder {
    id: String,
    treeview: TreeView,
    entry: Entry,
    shift_focus_callbacks: Rc<RefCell<Vec<Box<dyn Fn()>>>>
}

impl Folder {
    pub fn new(builder: &Builder, id: &str) -> Self {
        let treeview : TreeView = builder.object(&(id.to_string() + "-folder").to_string()).unwrap();
        let entry: Entry = builder.object(&(id.to_string() + "-entry").to_string()).unwrap();
        let shift_focus_callbacks = Rc::new(RefCell::new(Vec::new()));

        // append_column(&treeview, 0);
        // append_column(&treeview, 1);
        // append_column(&treeview, 2);
        // let model = create_and_fill_model();
        // treeview.set_model(Some(&model));

        let folder = Self {
            id: id.to_string(),
            treeview,
            entry,
            shift_focus_callbacks
        };
        folder.init();
        folder
    }

    pub fn focus(&self) {
        self.treeview.grab_focus();
    }

    pub fn connect_shift_focus<F: Fn() + 'static>(&self, f: F) {
        let mut b = self.shift_focus_callbacks.borrow_mut();
        b.push(Box::new(f));
    }

    fn init(&self) {
        let folder = self.clone();
        self.entry.connect_activate(move|_|{
            let text = folder.entry.text().to_string();
            println!("bin aktiviert: {}", text);
            folder.focus();
        });
        
        let folder_clone = self.clone();
        self.treeview.connect_key_press_event(move|_, k|{
            match k.keycode() {
                Some(23) if k.state() & ModifierType::SHIFT_MASK != ModifierType::SHIFT_MASK => {
                    let b = folder_clone.shift_focus_callbacks.borrow();
                    let was = &b[0];
                    let fun = was.as_ref();
                    fun();
                    Inhibit(true)
                },
                _ => {
                    Inhibit(false)
                }
            }
            
        });
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



