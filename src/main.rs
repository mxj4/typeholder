extern crate gtk;
extern crate gdk;
extern crate pango;

use gtk::prelude::*;

#[macro_use]
extern crate lazy_static;


mod range;

fn main() {
    println!("UNICODE_BLOCKS[15]: {:?}", range::UNICODE_BLOCKS[15]);
    println!("UNICODE_SCRIPTS[35]: {:?}", range::UNICODE_SCRIPTS[35]);

    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = gtk::Window::new(gtk::WindowType::Toplevel);

    window.set_title("Type Holder");
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(640, 480);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    let header_bar = gtk::HeaderBar::new();
    header_bar.set_show_close_button(true);
    let aliases_combo_box = gtk::ComboBoxText::new();
    let add_button = gtk::Button::new();
    let dup_button = gtk::Button::new();
    let del_button = gtk::Button::new();

    header_bar.pack_start(&aliases_combo_box);
    header_bar.pack_start(&add_button);
    header_bar.pack_start(&dup_button);
    header_bar.pack_start(&del_button);
    window.set_titlebar(&header_bar);

    window.show_all();
    gtk::main();
}
