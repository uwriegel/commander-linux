use gtk4::{
    Align, Box, Builder, HeaderBar, Image, Label, MenuButton, Orientation, 
    gio::MenuModel, pango::EllipsizeMode, prelude::{
        BoxExt, ButtonExt
    }
};

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

    let menuxml = r#"<?xml version="1.0" encoding="UTF-8"?>
    <interface>
      <menu id="app-menu">
        <section>
            <item>
                <attribute name="label">_Versteckte Dateien</attribute>
                <attribute name="action">app.showhidden</attribute>
            </item>
            <item>
                <attribute name="label">Beenden</attribute>
                <attribute name="action">app.quit</attribute>
            </item>
        </section>
      </menu>
    </interface>
    "#;

    let menu = Builder::from_string(menuxml)
        .object::<MenuModel>("app-menu")
        .expect("Expected menu model");

    let menubtn = MenuButton::builder()
        .icon_name("open-menu-symbolic")
        .receives_default(true)
        .menu_model(&menu)
        .build();

    vbox.append(&label);
    vbox.append(&sublabel);
    headerbar.set_title_widget(Some(&vbox));
    headerbar.pack_start(&previewbtn);
    headerbar.pack_end(&menubtn);
    headerbar
}