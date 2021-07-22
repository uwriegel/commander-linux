use gtk::{CellRendererText, TreeView, TreeViewColumn, prelude::{CellLayoutExt, TreeViewColumnExt, TreeViewExt}};

use super::{Processor, ProcessorType};

pub struct DirectoryProcessor;

impl Processor for DirectoryProcessor { 
    fn check(&self, processor_type: ProcessorType)-> bool { true }
    
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
        treeview.append_column(&column);
    
        let column = TreeViewColumn::new();
        let cell = CellRendererText::new();
        column.set_title(&format!("Datum"));
        column.set_resizable(true);
        column.set_expand(true);
        column.pack_start(&cell, true);
        treeview.append_column(&column);

        let column = TreeViewColumn::new();
        let cell = CellRendererText::new();
        column.set_title(&format!("Größe"));
        column.set_resizable(true);
        column.set_expand(true);
        column.pack_start(&cell, true);
        treeview.append_column(&column);
    }
}