use gdk::{Screen, prelude::SettingsExt};
use gio::{Settings, SimpleAction};
use gtk::{self, Application, ApplicationWindow, Builder, CssProvider, StyleContext, Widget, gdk, gio, prelude::*};

fn main() {
    let app = Application::new(Some("de.uriegel.commander"), Default::default());
    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let builder = Builder::new();
    builder.add_from_file("main.glade").unwrap();

    let window: ApplicationWindow = builder.object("window").unwrap();
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

