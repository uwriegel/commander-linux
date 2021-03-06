mod app;
mod folder;
mod processor;

use app::initialize_app;
use gtk::{Application, prelude::{ApplicationExt, ApplicationExtManual}};

fn main() {
    let application = Application::new(Some("de.uriegel.commander"), Default::default());
    application.connect_activate(initialize_app);
    application.run();
}

// TODO Fill ListBox with entry from DirectoryEntry
// TODO SingleSelection, red rectangle instead of blue filled rectangle
// TODO Folder: no focus rect in treeview
// TODO PixBuf file icon
// TODO Selection control
// TODO PixBuf selected
// TODO Sorting
// TODO Optional extension column
// TODO Restriction
// TODO History
// TODO Exif date with progress

