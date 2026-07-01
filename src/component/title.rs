use gtk::{Box, Label, prelude::*};

pub struct Title {
    pub root: Box,
    pub title_row: Box,
    pub title: Label,
    pub subtext: Label,
}

pub fn title() -> Title {
    let root = gtk::Box::new(gtk::Orientation::Vertical, 0);
    root.set_hexpand(true);

    let title_row = gtk::Box::new(gtk::Orientation::Horizontal, 8);
    title_row.set_valign(gtk::Align::Center);

    let icon = gtk::Label::new(Some("󰣇"));
    icon.set_widget_name("icon");

    let title = gtk::Label::new(Some("niri island"));
    title.set_hexpand(true);
    title.set_xalign(0.0);

    let subtext = gtk::Label::new(Some("noctalia compatible . layer-rule shell "));
    subtext.set_widget_name("subtext");
    subtext.set_xalign(0.0);
    subtext.set_ellipsize(gtk::pango::EllipsizeMode::End);
    subtext.set_max_width_chars(32);

    title_row.append(&icon);
    title_row.append(&title);
    root.append(&title_row);
    root.append(&subtext);

    Title {
        root,
        title_row,
        subtext,
        title,
    }
}
