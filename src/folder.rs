use std::{cell::RefCell, rc::Rc};

use gtk::{Builder, Entry, TreeView, gdk::ModifierType, prelude::*};

use crate::processor::{DefaultProcessor, Processor, check_processor};

#[derive(Clone)]
pub struct Folder {
    id: String,
    treeview: TreeView,
    entry: Entry,
    shift_focus_callbacks: Rc<RefCell<Vec<Box<dyn Fn()>>>>,
    processor: Rc<RefCell<Box<dyn Processor>>>
}

impl Folder {
    pub fn new(builder: &Builder, id: &str) -> Self {
        let treeview : TreeView = builder.object(&(id.to_string() + "-folder").to_string()).unwrap();
        let entry: Entry = builder.object(&(id.to_string() + "-entry").to_string()).unwrap();
        let shift_focus_callbacks = Rc::new(RefCell::new(Vec::new()));
        let processor: Rc<RefCell<Box<dyn Processor>>> = Rc::new(RefCell::new(Box::new(DefaultProcessor {})));

        let mut folder = Self {
            id: id.to_string(),
            treeview,
            entry,
            shift_focus_callbacks,
            processor
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

    fn init(&mut self) {
        let folder = self.clone();
        self.entry.connect_activate(move|_|{
            let text = folder.entry.text().to_string();
            folder.change_path(&text);
            folder.focus();
        });
        
        let folder_clone = self.clone();
        self.treeview.connect_key_press_event(move|_, k|{
            match k.keycode() {
                Some(23) if k.state() & ModifierType::SHIFT_MASK != ModifierType::SHIFT_MASK => {
                    let b = folder_clone.shift_focus_callbacks.borrow();
                    &b[0].as_ref()();
                    Inhibit(true)
                },
                _ => {
                    Inhibit(false)
                }
            }
            
        });
    }

    fn change_path(&self, path: &str) {
        let processor_changed = check_processor(&self.processor, path);
        let processor = self.processor.borrow_mut();
        if processor_changed {
            processor.prepare_treeview(&self.treeview);
        }
        processor.fill_items(&self.treeview, path.to_string());
    }
}




