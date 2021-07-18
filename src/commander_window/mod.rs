mod imp;

use glib::Object;
use gtk4::{ApplicationWindow, Widget, Window, prelude::*};
use gtk4::subclass::prelude::*;
use gtk4::Application;
use gtk4::{gio, glib};

glib::wrapper! {
    pub struct CommanderWindow(ObjectSubclass<imp::CommanderWindow>)
        @extends Widget, Window, ApplicationWindow,
        @implements gio::ActionMap, gio::ActionGroup;
}

impl CommanderWindow {
    pub fn new(app: &Application) -> Self {
        let window: Self = Object::new(&[]).expect("Failed to create CommanderWindow");
        window.set_application(Some(app));
        window
    }

    pub fn save_window_size(&self) -> Result<(), glib::BoolError> {
        // Get `settings` from `imp::CommanderWindow`
        let settings = &imp::CommanderWindow::from_instance(self).settings;

        // Get the size of the window
        let size = self.default_size();

        // Get the window state from `settings`
        settings.set_int("window-width", size.0)?;
        settings.set_int("window-height", size.1)?;
        settings.set_boolean("is-maximized", self.is_maximized())?;

        Ok(())
    }

    fn load_window_size(&self) {
        // Get `settings` from `imp::CommanderWindow`
        let settings = &imp::CommanderWindow::from_instance(self).settings;

        // Set the window state in `settings`
        let width = settings.int("window-width");
        let height = settings.int("window-height");
        let is_maximized = settings.boolean("is-maximized");

        // Set the size of the window
        self.set_default_size(width, height);

        // If the window was maximized when it was closed, maximize it again
        if is_maximized {
            self.maximize();
        }
    }
}
