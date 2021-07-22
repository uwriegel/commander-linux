use std::{fs::read_dir, thread};
use gtk::glib;
use gtk::prelude::Continue;
use gtk::{CellRendererText, ListStore, TreeView, TreeViewColumn, glib::{MainContext, PRIORITY_DEFAULT, clone}, prelude::{CellLayoutExt, GtkListStoreExtManual, StaticType, ToValue, TreeViewColumnExt, TreeViewExt}};

use super::{Processor, ProcessorType};

pub struct DirectoryProcessor;

impl Processor for DirectoryProcessor { 
    fn check(&self, processor_type: &ProcessorType)-> bool { *processor_type == ProcessorType::Directory }
    
    fn prepare_treeview(&self, treeview: &TreeView) {
        for col in treeview.columns() {
            treeview.remove_column(&col);
        }

        let column = TreeViewColumn::new();
        let cell = CellRendererText::new();
        column.set_title(&format!("Name"));
        column.set_resizable(true);
        column.set_expand(true);
        column.pack_start(&cell, true);
        column.add_attribute(&cell, "text", 0);
        treeview.append_column(&column);
    
        let column = TreeViewColumn::new();
        let cell = CellRendererText::new();
        column.set_title(&format!("Datum"));
        column.set_resizable(true);
        column.set_expand(true);
        column.pack_start(&cell, true);
        column.add_attribute(&cell, "text", 1);
        treeview.append_column(&column);

        let column = TreeViewColumn::new();
        let cell = CellRendererText::new();
        column.set_title(&format!("Größe"));
        column.set_resizable(true);
        column.set_expand(true);
        column.pack_start(&cell, true);
        column.add_attribute(&cell, "text", 2);
        treeview.append_column(&column);
    }

    fn get_items(&self, treeview: &TreeView, path: String) {

        let (sender, receiver) = MainContext::channel(PRIORITY_DEFAULT);
        thread::spawn(move || {
            let dirs = read_dir(&path).unwrap();
            println!("Read: {:?}", dirs);
            let entries = vec!["Michel".to_string(), "Sara".to_string(), "Liam".to_string(), "Zelda".to_string(), "Neo".to_string(), "Octopus master".to_string()];
            sender.send(entries).ok();
        });

        receiver.attach( None, clone!(@weak treeview => @default-return Continue(false),
            move |entries | {
                let model = ListStore::new(&[u32::static_type(), String::static_type()]);
                for (i, entry) in entries.iter().enumerate() {
                    model.insert_with_values(None, &[(0, &(i as u32 + 1).to_value()), (1, &entry.to_string().to_value())]);
                }
                treeview.set_model(Some(&model));            
                Continue(true)
            }),
        );        
    }
}