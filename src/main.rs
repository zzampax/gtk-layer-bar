use gio::prelude::*;
use gtk4::gdk;
use gtk4::prelude::{ApplicationExt, *};
use gtk4_layer_shell::{Edge, Layer, LayerShell};

fn load_css() {
    let display = gdk::Display::default().expect("Could not get default display.");
    let provider = gtk4::CssProvider::new();
    let priority = gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION;

    provider.load_from_data(include_str!("../styles/style.css"));
    gtk4::style_context_add_provider_for_display(&display, &provider, priority);
}

// https://github.com/wmww/gtk-layer-shell/blob/master/examples/simple-example.c
fn activate(application: &gtk4::Application) {
    // Create a normal GTK window however you like
    let window = gtk4::ApplicationWindow::new(application);

    // Before the window is first realized, set it up to be a layer surface
    window.init_layer_shell();

    // Display above normal windows
    window.set_layer(Layer::Top);

    // Push other windows out of the way
    window.auto_exclusive_zone_enable();

    window.set_default_size(-1, 30);

    // The margins are the gaps around the window's edges
    // Margins and anchors can be set like this...
    window.set_margin(Edge::Left, 10);
    window.set_margin(Edge::Right, 10);
    window.set_margin(Edge::Top, 10);

    // ... or like this
    // Anchors are if the window is pinned to each edge of the output
    let anchors = [
        (Edge::Left, true),
        (Edge::Right, true),
        (Edge::Top, true),
        (Edge::Bottom, false),
    ];

    for (anchor, state) in anchors {
        window.set_anchor(anchor, state);
    }

    // Set up a widget
    //let label = gtk4::Label::new(Some(""));
    //label.set_markup("<span font_desc=\"10.0\">GTK Layer Shell example!</span>");
    //window.set_child(Some(&label));
    let content = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    content.set_hexpand(true);
    content.set_vexpand(true);
    //content.set_spacing(20);
    content.set_css_classes(&["content"]);

    let left_section = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    let right_section = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    let center_section = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);

    let label = gtk4::Label::new(Some("GTK Layer Shell example [LEFT]!"));
    left_section.append(&label);
    let label = gtk4::Label::new(Some("GTK Layer Shell example [CENTER]!"));
    center_section.append(&label);
    let label = gtk4::Label::new(Some("GTK Layer Shell example [RIGHT]!"));
    right_section.append(&label);

    content.append(&left_section);
    content.append(&center_section);
    content.append(&right_section);

    left_section.set_halign(gtk4::Align::Start);
    center_section.set_halign(gtk4::Align::Center);
    right_section.set_halign(gtk4::Align::End);

    window.set_child(Some(&content));
    window.show()
}

fn main() {
    let application = gtk4::Application::new(Some("sh.wmww.gtk-layer-example"), Default::default());

    application.connect_startup(|_| load_css());
    application.connect_activate(|app| {
        activate(app);
    });

    application.run();
}
