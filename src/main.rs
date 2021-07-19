use gdk::{Screen, prelude::SettingsExt};
use gio::{Settings, SimpleAction};
use gtk::{self, gio, gdk, Application, ApplicationWindow, CssProvider, StyleContext, prelude::*};

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

    let window = ApplicationWindow::new(app);
    let settings = gio::Settings::new("de.uriegel.commander");
    let width = settings.int("window-width");
    let height = settings.int("window-height");
    let is_maximized = settings.boolean("is-maximized");
    window.set_default_size(width, height);
    if is_maximized {
        window.maximize();
    }

    window.connect_destroy(|win|{
        let settings = Settings::new("de.uriegel.commander");
        let size = win.default_size();
        settings.set_int("window-width", size.0).err();
        settings.set_int("window-height", size.1).err();
        settings.set_boolean("is-maximized", win.is_maximized()).err();
    });
    let provider = CssProvider::new();
    provider.load_from_data(".title {
        font-weight: bold;        
    }
    .title:backdrop {
        opacity: 0.6;
    }   
    .subtitle {y
        opacity: 0.6;
        font-size: 12px;
    }".as_bytes()).err();
    StyleContext::add_provider_for_screen(
        &Screen::default().expect("Error initializing gtk css provider."), 
        &provider, 
        gtk::STYLE_PROVIDER_PRIORITY_USER);

    window.present();
}

// TODO F3 show vgrid
// TODO hgrid
// TODO ListView in lef hgrid
// TODO TreeView
