use gtk::TreeView;

pub mod directory;

pub enum ProcessorType {
    Directory
}

pub trait Processor {
    fn check(&self, _processor_type: ProcessorType)-> bool { false }
    fn prepare_treeview(&self, _processor_type: &TreeView) { }
}

pub struct DefaultProcessor;

impl Processor for DefaultProcessor { }