use std::{cell::RefCell, rc::Rc};

use gtk::{TreeView, prelude::TreeViewExt};

use self::directory::DirectoryProcessor;

pub mod directory;

#[derive(PartialEq)]
pub enum ProcessorType {
    Default,
    Directory
}

pub trait Processor {
    fn check(&self, processor_type: &ProcessorType)-> bool { *processor_type == ProcessorType::Default }
    fn prepare_treeview(&self, treeview: &TreeView) { 
        for col in treeview.columns() {
            treeview.remove_column(&col);
        }
    }
    fn get_items(&self, _treeview: &TreeView, _path: String) { }
}
pub struct DefaultProcessor;

impl Processor for DefaultProcessor { }

pub fn check_processor(processor: &Rc<RefCell<Box<dyn Processor>>>, path: &str)->bool {
    let processor_type = get_processor_type(path);

    let check_processor = || {
        let processor = processor.borrow_mut();
        processor.check(&processor_type)
    };

    if !check_processor() {
        processor.replace(create_processor(processor_type));
        true
    } else  {
        false
    }
}

fn get_processor_type(path: &str)->ProcessorType {
    match path {
        "root" => ProcessorType::Default,
        _ if path.starts_with("/") => ProcessorType::Directory,
        _ => ProcessorType::Default,
    }
}

fn create_processor(processor_type: ProcessorType)->Box<dyn Processor> {
    match processor_type {
        ProcessorType::Default => Box::new(DefaultProcessor{ }),
        ProcessorType::Directory => Box::new(DirectoryProcessor{ })
    }
}