use core::fmt;
use std::time::UNIX_EPOCH;
use std::{fs::read_dir, thread};
use gtk::glib;
use gtk::prelude::Continue;
use gtk::{CellRendererText, ListStore, TreeView, TreeViewColumn, glib::{MainContext, PRIORITY_DEFAULT, clone}, prelude::{CellLayoutExt, GtkListStoreExtManual, StaticType, ToValue, TreeViewColumnExt, TreeViewExt}};
use lexical_sort::natural_lexical_cmp;

use super::{Processor, ProcessorType};

struct DirItem {
    name: String,
    is_hidden: bool
}
pub struct FileItem {
    name: String,
    is_hidden: bool,
    time: u128,
    size: u64
}

enum Item {
    DirItem(DirItem),
    FileItem(FileItem)
}

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

    fn fill_items(&self, treeview: &TreeView, path: String) {

        let (sender, receiver) = MainContext::channel(PRIORITY_DEFAULT);
        thread::spawn(move || {
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

pub struct Error {
    pub message: String
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({})", self.message)
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error {message: format!("read_dir failed: {}", error)}
    }
}

fn get_items(path: String, suppress_hidden: bool)->Result<Vec<Item>, Error> {
    let entries = read_dir(path.clone())?;
    //event_sinks.set_request(id, true);
    let (dirs, files): (Vec<_>, Vec<_>) = entries
        .filter_map(|entry| {
            entry.ok()
                .and_then(|entry| { match entry.metadata().ok() {
                    Some(metadata) => Some((entry, metadata)),
                    None => None
                }})
                .and_then(|(entry, metadata)| {
                    let name = String::from(entry.file_name().to_str().unwrap());
                    let is_hidden = is_hidden(&path, &name);
                    Some(match metadata.is_dir() {
                        true => Item::DirItem(DirItem {
                            name,
                            is_hidden
                        }),
                        false => Item::FileItem(FileItem {
                            name,
                            is_hidden,
                            time: metadata.modified().unwrap().duration_since(UNIX_EPOCH).unwrap().as_millis(),
                            size: metadata.len()
                        })
                    })
                })
                .and_then(get_supress_hidden(suppress_hidden))
        })
        .partition(|entry| if let Item::DirItem(_) = entry { true } else {false });
    let mut dirs: Vec<Item> = dirs
        .into_iter()
        .filter_map(|ft|if let Item::DirItem(dir) = ft {Some(Item::DirItem(dir))} else {None})
        .collect();
    dirs.sort_by(|a, b| {
        let aname = match a {
            Item::DirItem(n) => &n.name,
            _ => ""
        };
        let bname = match b {
            Item::DirItem(n) => &n.name,
            _ => ""
        };
        natural_lexical_cmp(aname, bname)
    });
    let mut files: Vec<Item> = files
        .into_iter()
        .filter_map(|ft|if let Item::FileItem(file) = ft {Some(Item::FileItem(file))} else {None})
        .collect();
    files.sort_by(|a, b| {
        let aname = match a {
            Item::FileItem(n) => &n.name,
            _ => ""
        };
        let bname = match b {
            Item::FileItem(n) => &n.name,
            _ => ""
        };
        natural_lexical_cmp(aname, bname)
    });
    dirs.append(&mut files);
    Ok(dirs)
}

fn is_hidden(_: &str, name: &str)->bool {
    name.as_bytes()[0] == b'.' && name.as_bytes()[1] != b'.'
}

fn get_supress_hidden(supress: bool) -> fn (Item)->Option<Item> {
    if supress {|file_type| {
        match file_type {
            Item::FileItem(ref file) => if file.is_hidden { None } else { Some(file_type) }
            Item::DirItem(ref file) => if file.is_hidden { None} else { Some(file_type) }
        }
    }} else { |e| Some(e) }
}
