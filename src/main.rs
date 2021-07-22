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

// TODO Check path!
// TODO Fill ListBox with entry from DirectoryEntry
// TODO SingleSelection, red rectangle instead of blue filled rectangle
// TODO Folder: no focus rect in treview
// TODO PixBuf file icon

