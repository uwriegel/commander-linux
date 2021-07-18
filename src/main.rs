mod commander_window;
mod headerbar;

use glib::clone;
use gtk4::gdk::Display;
use gtk4::{CssProvider, StyleContext, prelude::*};
use gtk4::{self, glib, Application};

use crate::commander_window::CommanderWindow;

fn main() {
    let app = Application::new(Some("de.uriegel.commander"), Default::default());
    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
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

// TODO HeaderBar: Button Preview, Hamburger Button with menu
// TODO ListView
// TODO TreeView
