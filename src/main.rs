mod commander_window;
mod headerbar;

use glib::clone;
use gtk4::gdk::Display;
use gtk4::gio::SimpleAction;
use gtk4::{CssProvider, StyleContext, prelude::*};
use gtk4::{self, glib, Application};

use crate::commander_window::CommanderWindow;

fn main() {
    let app = Application::new(Some("de.uriegel.commander"), Default::default());
    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let initial_bool_state = false.to_variant();
    let action = SimpleAction::new_stateful("viewer", None, &initial_bool_state);
    action.connect_change_state(move |a, s| {
        match s {
            Some(val) => {
                a.set_state(val);
                match val.get::<bool>(){
                    Some(show_viewer) => println!("show_viewer {}", show_viewer),
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


    let window = CommanderWindow::new(app);
    let headerbar = headerbar::create();
    window.set_titlebar(Some(&headerbar));

    let provider = CssProvider::new();
    provider.load_from_data(".title {
        font-weight: bold;        
    }
    .title:backdrop {
        opacity: 0.6;
    }   
    .subtitle {
        opacity: 0.6;
        font-size: 12px;
    }".as_bytes());
    StyleContext::add_provider_for_display(
        &Display::default().expect("Error initializing gtk css provider."), 
        &provider, 
        gtk4::STYLE_PROVIDER_PRIORITY_USER);

    window.present();
}

// TODO F3 show vgrid
// TODO hgrid
// TODO ListView in lef hgrid
// TODO TreeView
