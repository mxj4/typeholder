extern crate gtk;
extern crate gdk;
extern crate pango;
extern crate itertools;

use gtk::prelude::*;
use gtk::{Window, WindowPosition, WindowType, HeaderBar, StackSwitcher, ToggleButton, Image,
          IconSize, Paned, Orientation, TreeStore, TreeView, TreeViewColumn, CellRendererText,
          ListBox, ListBoxRow};

#[macro_use]
extern crate lazy_static;

mod range;
mod family;

fn append_text_column(tree: &TreeView) {
    let column = TreeViewColumn::new();
    let cell = CellRendererText::new();

    column.pack_start(&cell, true);
    column.add_attribute(&cell, "text", 0);
    tree.append_column(&column);
}

fn main() {
    // debug
    for i in family::read_available_families() {
        println!("{}", i.name);
    }
    println!("UNICODE_BLOCKS[15]: {:?}", range::UNICODE_BLOCKS[15]);
    println!("UNICODE_SCRIPTS[35]: {:?}", range::UNICODE_SCRIPTS[35]);

    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = Window::new(WindowType::Toplevel);
    window.set_wmclass("Typeholder", "Typeholder");
    window.set_title("Typeholder");
    window.set_position(WindowPosition::Center);
    window.set_default_size(640, 480);
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    let header_bar = HeaderBar::new();
    header_bar.set_show_close_button(true);

    let paned = Paned::new(Orientation::Horizontal);

    let switcher = StackSwitcher::new();

    let stack = gtk::Stack::new();

    let aliases_tree = TreeView::new();
    let aliases_store = TreeStore::new(&[String::static_type()]);
    aliases_tree.set_model(Some(&aliases_store));
    aliases_tree.set_headers_visible(false);
    append_text_column(&aliases_tree);
    for i in &["sans-serif", "serif", "monospace"] {
        let iter = aliases_store.insert_with_values(None, None, &[0], &[&format!("{}", i)]);
        for j in 0..3 {
            aliases_store.insert_with_values(
                Some(&iter),
                None,
                &[0],
                &[&format!("Test Family {}", j)],
            );
        }
    }
    aliases_tree.expand_all();

    let charsets_tree = TreeView::new();
    let charsets_store = TreeStore::new(&[String::static_type()]);
    charsets_tree.set_model(Some(&charsets_store));
    charsets_tree.set_headers_visible(false);

    stack.add_titled(&aliases_tree, "aliases", "Aliases");
    stack.add_titled(&charsets_tree, "charsets", "Charsets");

    switcher.set_stack(&stack);

    let search_button_image =
        Image::new_from_icon_name("edit-find-symbolic", IconSize::Menu.into());
    let search_button = ToggleButton::new();
    search_button.set_image(&search_button_image);

    let fonts_list = ListBox::new();

    paned.add1(&stack);
    paned.add2(&fonts_list);
    paned.set_position(280);

    header_bar.pack_start(&switcher);
    header_bar.pack_end(&search_button);

    window.set_titlebar(&header_bar);
    window.add(&paned);

    window.show_all();
    gtk::main();
}
