mod processor;

use gdk::{Screen, prelude::SettingsExt};
use gio::{Settings, SimpleAction};
use gtk::{self, Application, ApplicationWindow, Builder, CellRendererText, CssProvider, ListStore, StyleContext, TreeView, TreeViewColumn, Widget, gdk, gio, prelude::*};

fn append_column(tree: &TreeView, id: i32) {
    let column = TreeViewColumn::new();
    let cell = CellRendererText::new();

    column.set_title(&format!("Spalte {}", id));
    column.set_resizable(true);
    column.set_expand(true);
    column.pack_start(&cell, true);
    // Association of the view's column with the model's `id` column.
    column.add_attribute(&cell, "text", id);
    tree.append_column(&column);
}

fn create_and_fill_model() -> ListStore {
    // Creation of a model with two rows.
    let model = ListStore::new(&[u32::static_type(), String::static_type()]);
    
    // Filling up the tree view.
    let entries = &["Michel", "Sara", "Liam", "Zelda", "Neo", "Octopus master"];
    for (i, entry) in entries.iter().enumerate() {
        model.insert_with_values(None, &[(0, &(i as u32 + 1).to_value()), (1, &entry.to_string().to_value())]);
    }
    model
}


fn main() {

    processor::test();
    processor::directory::testdir();

    let app = Application::new(Some("de.uriegel.commander"), Default::default());
    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let builder = Builder::new();
    builder.add_from_file("main.glade").unwrap();

    let window: ApplicationWindow = builder.object("window").unwrap();
    let left_commander: TreeView = builder.object("leftCommander").unwrap();
    let right_commander: TreeView = builder.object("rightCommander").unwrap();
    append_column(&left_commander, 0);
    append_column(&left_commander, 1);
    append_column(&left_commander, 2);
    let model = create_and_fill_model();
    left_commander.set_model(Some(&model));

    append_column(&right_commander, 0);
    append_column(&right_commander, 1);
    append_column(&right_commander, 2);
    let model = create_and_fill_model();
    right_commander.set_model(Some(&model));
    
    let viewer: Widget = builder.object("viewer").unwrap();
    viewer.set_visible(false);
    app.add_window(&window);

    let initial_bool_state = false.to_variant();
    let action = SimpleAction::new_stateful("viewer", None, &initial_bool_state);
    action.connect_change_state(move |a, s| {
        match s {
            Some(val) => {
                a.set_state(val);
                match val.get::<bool>(){
                    Some(show_viewer) => {
                        viewer.set_visible(show_viewer);
                        println!("show_viewer {}", show_viewer)
                    },
                    None => println!("Could not set ShowViewer, could not extract from variant")
                }
            },
            None => println!("Could not set action")
        }
    });
    app.add_action(&action);
    app.set_accels_for_action("app.viewer", &["F3"]);

    let action = SimpleAction::new_stateful("showhidden", None, &initial_bool_state);
    action.connect_change_state(move |a, s| {
        match s {
            Some(val) => {
                a.set_state(val);
                match val.get::<bool>(){
                    Some(show_hidden) => println!("show_hidden {}", show_hidden),
                    None => println!("Could not set ShowHidden, could not extract from variant")
                }
            },
            None => println!("Could not set action")
        }
    });
    app.add_action(&action);
    app.set_accels_for_action("app.showhidden", &["<Ctrl>H"]);

    let settings = gio::Settings::new("de.uriegel.commander");
    let width = settings.int("window-width");
    let height = settings.int("window-height");
    let is_maximized = settings.boolean("is-maximized");
    window.set_default_size(width, height);
    if is_maximized {
        window.maximize();
    }

    window.connect_configure_event(|win, _|{
        let settings = Settings::new("de.uriegel.commander");
        let size = win.size();
        settings.set_int("window-width", size.0).err();
        settings.set_int("window-height", size.1).err();
        settings.set_boolean("is-maximized", win.is_maximized()).err();
        false
    });
    let provider = CssProvider::new();
    provider.load_from_data(".test {
        background-color: yellow;        
    }
    .hidden {
        visible: false;
    }".as_bytes()).err();
    StyleContext::add_provider_for_screen(
        &Screen::default().expect("Error initializing gtk css provider."), 
        &provider, 
        gtk::STYLE_PROVIDER_PRIORITY_USER);

    window.present();
}

// TODO Custom Application with Boxes for class leftFolder and RightFolder
// TODO Folder: class with processor
// TODO class Processor with Box<Trait>
// TODO Fill ListBox with entry from DirectoryEntry
// TODO SingfelSelection, red rectangle instead of blue filled rectangle
// TODO PixBuf file icon
// TODO Tab control
