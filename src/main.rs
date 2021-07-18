use glib::clone;
use gtk4::gdk::Display;
use gtk4::{CssProvider, StyleContext, prelude::*};
use gtk4::{self, glib, Application, ApplicationWindow, Button};

fn main() {
    let app = Application::new(Some("de.uriegel.commander"), Default::default());
    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Commander")
        .default_width(600)
        .default_height(800)
        .build();

    let provider = CssProvider::new();
    provider.load_from_data("window {
        background-color: orange;        
        color: blue;
    }
    *:focus {
        outline-color: red										;
        outline-style: solid;
        outline-width: 1px;
        outline-offset: -1px;
    }".as_bytes());
    StyleContext::add_provider_for_display(
        &Display::default().expect("Error initializing gtk css provider."), 
        &provider, 
        gtk4::STYLE_PROVIDER_PRIORITY_USER);

    let button = Button::with_label("Monitor");

    // Set the button margins
    button.set_margin_top(18);
    button.set_margin_bottom(18);
    button.set_margin_start(18);
    button.set_margin_end(18);

    // Connect callback
    button.connect_clicked(clone!(@weak window => move |button| {
        // Set the label to "Hello World!" after the button has been clicked on
        button.set_label("Hello World!");
        let display = window.display();
        let name = display.name();
        println!("Monitor: {}", name)
    }));

    window.set_child(Some(&button));
    window.present();
}

// TODO Saving window state
// TODO HeaderBar
// TODO ListView
// TODO TreeView
