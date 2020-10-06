extern crate gtk;
extern crate gio;

use gtk::prelude::*;
use gio::prelude::*;

use gtk::{Application, ApplicationWindow, Button};

fn main() {
    let application = Application::new(
        Some("com.github.gtk-rs.examples.basic"),
        Default::default(),
    ).expect("failed to initialize GTK application");

    application.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("Handy GCS");
        window.set_default_size(350, 70);

        let button = Button::with_label("Click me!");
        window.add(&button);
        window.show_all();

        button.connect_clicked(move |_| {

            println!("Size: {:?}", window.get_size());
        });
    });

    application.run(&[]);
}
