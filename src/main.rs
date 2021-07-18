use glib::clone;
use gtk4::prelude::*;
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

// TODO css styling
// TODO Saving window state
// TODO HeaderBar
// TODO ListView
// TODO TreeView
