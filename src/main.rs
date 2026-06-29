use gtk::{
    gdk,
    gio::prelude::{ApplicationExt, ApplicationExtManual},
    prelude::{BoxExt, GtkWindowExt, WidgetExt},
};
use gtk4_layer_shell::{Edge, Layer, LayerShell};

const APP_ID: &str = "dev.nixos.NiriIsland";
const NAMESPACE: &str = "niri-island";
const CSS: &str = r#""#;

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
    window.set_margin(Edge::Top, 10);

    window.set_exclusive_zone(0);

    let root = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    root.set_widget_name("niri-island");
    root.set_halign(gtk::Align::Center);
    root.set_valign(gtk::Align::Center);

    let icon = gtk::Label::new(Some("󰣇"));
    icon.set_widget_name("icon");

    let texts = gtk::Box::new(gtk::Orientation::Vertical, 0);

    let title = gtk::Label::new(Some("niri island"));
    title.set_xalign(0 as f32);

    let subtext = gtk::Label::new(Some("noctalia compatible . layer-rule shell "));
    subtext.set_widget_name("subtext");
    subtext.set_xalign(0 as f32);

    texts.append(&title);
    texts.append(&subtext);

    root.append(&icon);
    root.append(&texts);

    let motion = gtk::EventControllerMotion::new();

    {
        let root = root.clone();
        motion.connect_enter(move |_, _, _| {
            root.add_css_class("expanded");
        });
    }
    {
        let root = root.clone();
        motion.connect_leave(move |_| {
            root.remove_css_class("expanded");
        });
    }
    root.add_controller(motion);

    {
        let title = title.clone();
        gtk::glib::timeout_add_seconds_local(1, move || {
            title.set_text("niri + coctalia");
            gtk::glib::ControlFlow::Continue
        });
    }
    window.set_child(Some(&root));
    window.present();
}
fn main() -> gtk::glib::ExitCode {
    let app = gtk::Application::new(Some(APP_ID), gtk::gio::ApplicationFlags::empty());
    app.connect_activate(activate);
    app.run()
}
