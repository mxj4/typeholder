#![feature(conservative_impl_trait)]

extern crate glib;
extern crate gtk;
//extern crate gdk;
extern crate pango;
extern crate itertools;
extern crate sxd_document;

use std::ops::Deref;
use std::time::Instant;
use itertools::Itertools;
use glib::{Continue, idle_add, timeout_add, timeout_add_seconds};
use gtk::prelude::*;
use gtk::{WidgetExt, Window, WindowPosition, WindowType, HeaderBar, StackSwitcher, ToggleButton,
          Image, IconSize, Paned, Orientation, TreeStore, TreeView, TreeViewColumn,
          CellRendererText, ListBox, ListBoxRow, Label, Viewport, ScrolledWindow, PolicyType};

#[macro_use]
extern crate lazy_static;

mod alias;
mod consts;
mod range;
mod family;
mod config;
mod deserialization;

fn append_text_column(tree: &TreeView) {
    let column = TreeViewColumn::new();
    let cell = CellRendererText::new();

    column.pack_start(&cell, true);
    column.add_attribute(&cell, "text", 0);
    tree.append_column(&column);
}

fn main() {
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

    // debug
    println!("UNICODE_BLOCKS[15]: {:?}", range::UNICODE_BLOCKS[15]);
    println!("UNICODE_SCRIPTS[35]: {:?}", range::UNICODE_SCRIPTS[35]);

    let available_families = deserialization::list_families(&window.create_pango_context().expect(
        "Failed to create Pango context!",
    ));

    let fc_config = deserialization::parse_or_default(&available_families);
    println!("Parsed config: {:?}", fc_config);



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
    for i in &fc_config.aliases {
        let iter = aliases_store.insert_with_values(None, None, &[0], &[&format!("{}", i.name)]);
        for j in &i.prefer_list {
            aliases_store.insert_with_values(Some(&iter), None, &[0], &[&j.borrow().name]);
        }
    }
    aliases_tree.expand_all();

    let charsets_tree = TreeView::new();
    let charsets_store = TreeStore::new(&[String::static_type()]);
    charsets_tree.set_model(Some(&charsets_store));
    charsets_tree.set_headers_visible(false);
    append_text_column(&charsets_tree);
    for i in &fc_config.scan_matches {
        let iter =
            charsets_store.insert_with_values(None, None, &[0], &[&format!("{}", i.borrow().name)]);
        for j in &i.borrow().stripped_ranges {
            let (range_name, range_type, range_value) = match j {
                &range::Range::Block {
                    name: ref n,
                    code_points: ref v,
                } => (n, "Block", format!("0x{:x}..0x{:x}", v.0, v.1)),
                &range::Range::Script {
                    name: ref n,
                    code_points: ref v,
                } => (
                    n,
                    "Script",
                    v.iter()
                        .map(|&(x, y)| format!("0x{:x}..0x{:x}", x, y))
                        .join(", "),
                ),
                &range::Range::Custom {
                    name: ref n,
                    code_points: ref v,
                } => (n, "Custom", format!("0x{:x}..0x{:x}", v.0, v.1)),
            };
            charsets_store.insert_with_values(
                Some(&iter),
                None,
                &[0],
                &[&format!("{}: {} {}", range_name, range_type, range_value)],
            );
        }
    }
    charsets_tree.expand_all();

    stack.add_titled(&aliases_tree, "aliases", "Aliases");
    stack.add_titled(&charsets_tree, "charsets", "Charsets");

    switcher.set_stack(&stack);

    let search_button_image =
        Image::new_from_icon_name("edit-find-symbolic", IconSize::Menu.into());
    let search_button = ToggleButton::new();
    search_button.set_image(&search_button_image);

    let fonts_scrolled = ScrolledWindow::new(None, None);
    fonts_scrolled.set_policy(PolicyType::Never, PolicyType::Automatic);
    let fonts_view = Viewport::new(None, None);
    let fonts_list = ListBox::new();
    for fam in &available_families {
        let row = ListBoxRow::new();
        let label = Label::new(None);
        label.set_markup(format!("{}", &fam.borrow().deref().name).as_str());
        // todo
        //label.set_markup(format!(
        // "<span font_family=\"{}\">{}</span>", &fam.name, &fam.name
        // ).as_str());
        row.add(&label);

        fonts_list.add(&row);
    }
    fonts_view.add(&fonts_list);
    fonts_scrolled.add(&fonts_view);

    paned.add1(&stack);
    paned.add2(&fonts_scrolled);
    paned.set_position(245);

    header_bar.pack_start(&switcher);
    header_bar.pack_end(&search_button);

    window.set_titlebar(&header_bar);
    window.add(&paned);

    // todo
    let start = Instant::now();
    window.show_all();
    println!(
        "Init time: {} s",
        Instant::now().duration_since(start).as_secs()
    );

    glib::idle_add(|| Continue(false));
    gtk::main();
}
