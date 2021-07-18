use gtk4::{Align, Box, HeaderBar, Image, Label, MenuButton, Orientation, pango::EllipsizeMode, prelude::{BoxExt, ButtonExt}};

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
    let previewbtn = gtk4::ToggleButton::builder()
        .receives_default(true)
        .action_name("app.viewer")
        .build();
    let previewimg = Image::builder()
        .icon_name("document-print-preview")
        .build();
    previewbtn.set_child(Some(&previewimg));

    let menubtn = MenuButton::builder()
        .icon_name("open-menu-symbolic")
        .receives_default(true)
        //.popover(popover)
        .build();

    vbox.append(&label);
    vbox.append(&sublabel);
    headerbar.set_title_widget(Some(&vbox));
    headerbar.pack_start(&previewbtn);
    headerbar.pack_end(&menubtn);
    headerbar
}