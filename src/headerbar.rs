use gtk4::{Align, Box, HeaderBar, Label, Orientation, pango::EllipsizeMode, prelude::BoxExt};

pub fn create()->HeaderBar {
    let headerbar = HeaderBar::builder()
        .build();

    let vbox = Box::builder()
        .orientation(Orientation::Vertical)
        .valign(Align::Center)
        .build();
    let label = Label::builder()
        .label("Commander")
        .css_classes(vec!["title".to_string()])
        .ellipsize(EllipsizeMode::End)
        .build();
    let sublabel = Label::builder()
        .label("Subtitle")
        .css_classes(vec!["subtitle".to_string()])
        .ellipsize(EllipsizeMode::End)
        .build();

    vbox.append(&label);
    vbox.append(&sublabel);
    headerbar.set_title_widget(Some(&vbox));
    headerbar
}