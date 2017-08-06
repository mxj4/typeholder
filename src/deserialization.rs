use alias::Alias;
use config::Config;
use consts::*;
use itertools::Itertools;
use family::Family;
use glib::functions;
use pango::Context;
use pango::ContextExt;
use pango::FontMapExt;
use pango::FontFamilyExt;
use range::Range;
use sxd_document::dom;
use sxd_document::dom::Comment;
use sxd_document::dom::ChildOfElement;
use sxd_document::dom::Element;
use sxd_document::dom::Text;
use sxd_document::parser;

use std::cell::RefCell;
use std::fs::File;
use std::io::Read;
use std::ops::Deref;


pub fn list_families(context: &Context) -> Vec<RefCell<Family>> {
    match context.get_font_map() {
        Some(map) => {
            map.list_families()
                .iter()
                .filter_map(|x| x.get_name())
                .filter(|x| !["Sans", "Serif", "Monospace"].contains(&x.as_str()))
                .map(|x| {
                    RefCell::new(Family {
                        name: x,
                        stripped_ranges: vec![],
                    })
                })
                .collect()
        }
        None => vec![],
    }
}

pub fn parse_or_default<'a>(families: &'a Vec<RefCell<Family>>) -> Config<'a> {
    let fc_config_path = functions::get_user_config_dir()
        .expect("$XDG_CONFIG_HOME not set!")
        .join("fontconfig/fonts.conf");
    let config_parse = match File::open(fc_config_path.as_path()) {
        Ok(mut f) => {
            let mut buffer = String::new();
            f.read_to_string(&mut buffer).expect(
                "Failed to parse your fonts.conf file",
            );
            parser::parse(&buffer)
        }
        _ => parser::parse(DEFAULT_FONTS_CONF),
    };
    let config_package = match config_parse {
        Ok(package) => package,
        Err((_, errors)) => panic!("Error parsing fonts.conf!\n{}", errors.iter().join("\n")),
    };

    // scan matches collection
    let mut scan_matches: Vec<&'a RefCell<Family>> = vec![];
    // aliases collection
    let mut aliases: Vec<Alias<'a>> = vec![];

    {
        let doc = config_package.as_document();

        let old_root_element = doc.root().children()[0].element().expect(
            "Invalid XML root in the configuration file!",
        );

        // rest of dom collection
        let new_root_element = doc.create_element(old_root_element.name());
        for attr in old_root_element.attributes() {
            new_root_element.set_attribute_value(attr.name(), attr.value());
        }

        // group children to correct collections
        for child in old_root_element.children() {
            match child {
                ChildOfElement::Comment(x) if is_typeholder_comment(x) => {}
                ChildOfElement::Element(x) if prev_is_typeholder_comment(x) => {
                    if x.name().local_part() == "alias" {
                        aliases.push(parse_alias(x, families));
                    } else if x.name().local_part() == "match" &&
                               x.attribute_value("target").unwrap_or("") == "scan"
                    {
                        // todo
                    }
                }
                x => new_root_element.append_child(x),
            }
        }

        // replace old_root_element with new_root_element
        doc.root().append_child(new_root_element);
    }

    Config {
        scan_matches: scan_matches,
        aliases: aliases,
        residue: config_package,
    }
}

fn prev_is_typeholder_comment(x: Element) -> bool {
    match x.preceding_siblings().last() {
        Some(y) => {
            match y.comment() {
                Some(z) => is_typeholder_comment(z),
                None => false,
            }
        }
        None => false,
    }
}

fn is_typeholder_comment(x: Comment) -> bool {
    x.text().starts_with(TYPEHOLDER_COMMENT_PREFIX)
}

fn collect_scan_matches<'a>(
    root: dom::Root,
    families: &'a Vec<RefCell<Family>>,
) -> Vec<&'a RefCell<Family>> {
    // todo
    vec![]
}

fn parse_alias<'a>(e: Element, families: &'a Vec<RefCell<Family>>) -> Alias<'a> {
    let alias_name = checked_text(checked_child("family", e)).text();
    let p_list = children("family", checked_child("prefer", e))
        .filter_map(|x| {
            families.iter().find(|y| {
                y.borrow().deref().name == checked_text(x).text()
            })
        })
        .collect_vec();

    Alias {
        name: String::from(alias_name),
        prefer_list: p_list,
    }
}

fn checked_child<'a: 'd, 'd>(name: &'a str, e: Element<'d>) -> Element<'d> {
    child(name, e).expect(&format!(
        "Element {} has no {} child!",
        e.name().local_part(),
        name
    ))
}

fn child<'a: 'd, 'd>(name: &'a str, e: Element<'d>) -> Option<Element<'d>> {
    children(name, e).next()
}

fn children<'a: 'd, 'd>(name: &'a str, e: Element<'d>) -> impl Iterator<Item = Element<'d>> + 'd {
    e.children()
        .into_iter()
        .filter_map(|x| x.element())
        .filter(move |x| x.name().local_part() == name)
}

fn checked_text<'d>(e: Element<'d>) -> Text<'d> {
    text(e).expect(&format!("Element {} has no text!", e.name().local_part()))
}

fn text<'d>(e: Element<'d>) -> Option<Text<'d>> {
    e.children().into_iter().filter_map(|x| x.text()).next()
}
