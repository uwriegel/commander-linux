use gio::Settings;
use glib::signal::Inhibit;
use gtk4::{gio, glib};
use gtk4::{subclass::prelude::*, ApplicationWindow};

pub struct CommanderWindow {
    pub settings: Settings,
}

#[glib::object_subclass]
impl ObjectSubclass for CommanderWindow {
    const NAME: &'static str = "CommanderWindow";
    type Type = super::CommanderWindow;
    type ParentType = ApplicationWindow;

    fn new() -> Self {
        Self {
            settings: Settings::new("de.uriegel.commander"),
        }
    }
}
impl ObjectImpl for CommanderWindow {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);
        obj.load_window_size();
    }
}
impl WidgetImpl for CommanderWindow {}
impl WindowImpl for CommanderWindow {
    fn close_request(&self, obj: &Self::Type) -> Inhibit {
        if let Err(err) = obj.save_window_size() {
            println!("Failed to save window state, {}", &err);
        }
        Inhibit(false)
    }
}
impl ApplicationWindowImpl for CommanderWindow {}
