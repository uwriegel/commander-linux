mod app;
mod processor;

use app::App;

fn main() {
    let app = App::new();
    app.run();

    processor::test();
    processor::directory::testdir();
}


// TODO Custom Application with Boxes for class leftFolder and RightFolder
// TODO Folder: class with processor
// TODO class Processor with Box<Trait>
// TODO Fill ListBox with entry from DirectoryEntry
// TODO SingleSelection, red rectangle instead of blue filled rectangle
// TODO PixBuf file icon
// TODO Tab control
