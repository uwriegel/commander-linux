mod commander_window;

use glib::clone;
use gtk4::gdk::Display;
use gtk4::{CssProvider, StyleContext, prelude::*};
use gtk4::{self, glib, Application, Button};

use crate::commander_window::CommanderWindow;

fn main() {
    let app = Application::new(Some("de.uriegel.commander"), Default::default());
    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let window = CommanderWindow::new(app);

    let provider = CssProvider::new();
    provider.load_from_data("
    }".as_bytes());
    StyleContext::add_provider_for_display(
        &Display::default().expect("Error initializing gtk css provider."), 
        &provider, 
        gtk4::STYLE_PROVIDER_PRIORITY_USER);

    window.present();
}

// TODO ListView
// TODO TreeView
// TODO HeaderBar