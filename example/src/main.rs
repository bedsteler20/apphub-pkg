use client::{gio, gio::prelude::*, Client, GTransaction};
use gtk4::prelude::GtkWindowExt;

fn main() {
    let app = gtk4::Application::new(
        Some("dev.bedsteler20.test"),
        gio::ApplicationFlags::FLAGS_NONE,
    );
    app.connect_activate(|app| {
        let client = Client::new();
        let win = gtk4::Window::new();
        win.set_application(Some(app));
        win.present();

        println!("uwu");
        client.bind_dbus_signals();
        client.transactions.connect_items_changed(|store, a, b, c| {
            let i = store.item(a).unwrap().downcast::<GTransaction>().unwrap();
            i.connect_progress_notify(|se| {
                println!("Progress: {}", se.progress());
            });
        });
    });
    app.run();
}
