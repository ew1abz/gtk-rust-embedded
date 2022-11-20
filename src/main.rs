use gtk::prelude::*;
use gtk::{Label, Window, WindowType};
use std::process;

fn main() {
    const APP_NAME: &str = "Gtk-Rust Example";
    glib::set_program_name(APP_NAME.into());
    glib::set_application_name(APP_NAME);

    if gtk::init().is_err() {
        eprintln!("failed to initialize GTK Application");
        process::exit(1);
    }

    let window = Window::new(WindowType::Toplevel);

    //Destroy window on exit
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    window.set_title(APP_NAME);
    window.set_default_size(350, 300);

    let label = Label::new(Some("Rust is cool!"));

    window.add(&label);
    window.show_all();

    // Thread will block here until the application is quit
    gtk::main();
}
