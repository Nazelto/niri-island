use gtk::{
    RevealerTransitionType, gdk,
    gio::prelude::{ApplicationExt, ApplicationExtManual},
    prelude::{BoxExt, GtkWindowExt, WidgetExt},
};
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use std::{cell::RefCell, rc::Rc};

use crate::{
    component::title::title,
    task::{source_task, spectrum_task},
};

mod app;
mod audio;
mod component;
mod media;
mod task;
const APP_ID: &str = "dev.nixos.NiriIsland";
const NAMESPACE: &str = "niri-island";
const CSS: &str = include_str!("css/style.css");
fn load_css() {
    let provider = gtk::CssProvider::new();
    provider.load_from_data(CSS);

    if let Some(display) = gdk::Display::default() {
        gtk::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}
fn activate(app: &gtk::Application) {
    load_css();
    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .title("niri-island")
        .default_width(360)
        .default_height(58)
        .decorated(false)
        .resizable(false)
        .build();
    window.add_css_class("island-window");
    //  普通GTK窗口 变成 layer-shell surface
    window.init_layer_shell();
    // 给 niri layer-rule 匹配
    window.set_namespace(Some(NAMESPACE));
    // overlay 压过全屏窗口
    window.set_layer(Layer::Top);
    // 不抢键盘焦点
    window.set_keyboard_mode(gtk4_layer_shell::KeyboardMode::None);

    // 顶部居中: 只 anchor Top,
    window.set_anchor(Edge::Top, true);
    window.set_anchor(Edge::Left, true);
    window.set_margin(Edge::Top, 10);

    window.set_exclusive_zone(0);

    let root = gtk::Box::new(gtk::Orientation::Vertical, 0);
    root.set_widget_name("island");
    root.set_halign(gtk::Align::Center);
    root.set_valign(gtk::Align::Center);

    let top_row = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    top_row.set_widget_name("island-top-row");
    top_row.set_halign(gtk::Align::Center);
    top_row.set_valign(gtk::Align::Center);

    let spectrum_row = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    spectrum_row.set_widget_name("spectrum-row");
    spectrum_row.set_halign(gtk::Align::Center);
    spectrum_row.set_valign(gtk::Align::Center);
    let title = title();
    top_row.append(&title.root);

    let spectrum_values = Rc::new(RefCell::new(vec![0.0; crate::audio::BAR_COUNT as usize]));
    let top_spectrum = component::spectrum::spectrum(spectrum_values.clone(), 128, 14);
    let expanded_spectrum = component::spectrum::spectrum(spectrum_values.clone(), 256, 28);

    let top_spectrum_revealer = gtk::Revealer::builder()
        .transition_type(RevealerTransitionType::SlideLeft)
        .transition_duration(180)
        .reveal_child(true)
        .child(&top_spectrum)
        .build();
    let expanded_spectrum_revealer = gtk::Revealer::builder()
        .transition_type(RevealerTransitionType::SlideDown)
        .transition_duration(180)
        .reveal_child(false)
        .child(&expanded_spectrum)
        .build();

    top_row.append(&top_spectrum_revealer);
    spectrum_row.append(&expanded_spectrum_revealer);
    root.append(&top_row);
    root.append(&spectrum_row);

    let motion = gtk::EventControllerMotion::new();

    {
        let subtext = title.subtext.clone();
        let title_row = title.title_row.clone();
        let root = root.clone();
        let top_spectrum_revealer = top_spectrum_revealer.clone();
        let expanded_spectrum_revealer = expanded_spectrum_revealer.clone();
        motion.connect_enter(move |_, _, _| {
            root.add_css_class("expanded");
            top_spectrum_revealer.set_reveal_child(false);
            expanded_spectrum_revealer.set_reveal_child(true);
            subtext.set_max_width_chars(64);
            title_row.set_halign(gtk::Align::Center);
        });
    }
    {
        let subtext = title.subtext.clone();

        let title_row = title.title_row.clone();
        let root = root.clone();
        let top_spectrum_revealer = top_spectrum_revealer.clone();
        let expanded_spectrum_revealer = expanded_spectrum_revealer.clone();
        motion.connect_leave(move |_| {
            root.remove_css_class("expanded");
            expanded_spectrum_revealer.set_reveal_child(false);
            top_spectrum_revealer.set_reveal_child(true);
            subtext.set_max_width_chars(32);
            title_row.set_halign(gtk::Align::Start);
        });
    }
    root.add_controller(motion);
    // {
    //     let title = title.clone();
    //     gtk::glib::timeout_add_seconds_local(1, move || {
    //         title.set_text("niri + coctalia");
    //         gtk::glib::ControlFlow::Continue
    //     });
    // }
    window.set_child(Some(&root));
    window.present();
    let (tx, rx) = std::sync::mpsc::channel();
    let audio_source = std::env::var("NIRI_ISLAND_AUDIO_SOURCE").ok();

    audio::spawn_spectrum(tx, audio_source.clone());

    let top_spectrum = top_spectrum.clone();
    let expanded_spectrum = expanded_spectrum.clone();
    let spectrum_values = spectrum_values.clone();
    spectrum_task(vec![top_spectrum, expanded_spectrum], spectrum_values, rx);
    let fallback = audio_source
        .clone()
        .map(|source| format!("source:{}", source))
        .unwrap_or_else(|| "source: default monitor".to_owned());
    source_task(title.title, title.subtext, fallback);
}
fn main() -> gtk::glib::ExitCode {
    let app = gtk::Application::new(Some(APP_ID), gtk::gio::ApplicationFlags::empty());
    app.connect_activate(activate);
    app.run()
}
