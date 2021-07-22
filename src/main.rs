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



// TODO Custom Application with Boxes for class leftFolder and RightFolder
// TODO Folder: class with processor
// TODO class Processor with Box<Trait>
// TODO Fill ListBox with entry from DirectoryEntry
// TODO SingleSelection, red rectangle instead of blue filled rectangle
// TODO PixBuf file icon
// TODO Tab control
