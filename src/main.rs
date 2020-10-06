extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Builder, Button, Entry, Label};

use std::env::args;

fn build_connection(builder: &Builder) {
        let ip_entry: Entry = builder.get_object("IPEntry").unwrap();
    let port_entry: Entry = builder.get_object("PortEntry").unwrap();
    let connect_button: Button = builder.get_object("ConnectButton").unwrap();
    let connection_status_label: Label = builder
        .get_object("ConnectionStatusLabel")
        .unwrap();

    connect_button.connect_clicked(move |_| {
        let new_label = format!(
            "Connected {}:{}",
            ip_entry.get_text(),
            port_entry.get_text()
        );
        connection_status_label.set_text(&new_label);
    });

}

fn build_ui(app: &gtk::Application) {
    let glade_src = include_str!("handy.glade");
    let builder = Builder::from_string(glade_src);

    let main_window: ApplicationWindow = builder.get_object("MainWindow").unwrap();
    main_window.set_application(Some(app));

    build_connection(&builder);

    main_window.show_all();
}

fn main() {
    let application = Application::new(Some("com.github.erismik.HandyGCS"), Default::default())
        .expect("Failed to initialize GTK application");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}
