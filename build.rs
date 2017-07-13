#![feature(slice_patterns)]

use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::path::Path;

extern crate linked_hash_map;
use linked_hash_map::LinkedHashMap;

extern crate itertools;
use itertools::Itertools;


fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("ucd.rs");
    let mut f = File::create(&dest_path).unwrap();

    write(&mut f, generate_blocks_macro(read_lines("ucd/Blocks.txt")));
    write(
        &mut f,
        generate_scripts_macro(read_lines("ucd/Scripts.txt")),
    );
}

fn generate_blocks_macro(lines: Vec<String>) -> String {
    let codes = lines
        .iter()
        .map(|l| {
            let pair: Vec<&str> = l.splitn(2, "; ").collect();
            let range: Vec<&str> = pair[0].splitn(2, "..").collect();
            let name = pair[1];
            format!(
                "Range::Block {{ name: String::from(\"{}\"), code_points: (0x{}, 0x{}) }}",
                name,
                range[0],
                range[1]
            )
        })
        .join(&format!(",\n{}", " ".repeat(4 * 3)));

    format!(
        "
macro_rules! unicode_blocks {{
    () => (
        vec![
            {}
        ];
    )
}}
",
        codes
    )
}

fn generate_scripts_macro(lines: Vec<String>) -> String {
    let scripts: LinkedHashMap<&str, Vec<(&str, &str)>> = lines
        .iter()
        .filter_map(|l| l.splitn(2, ";").map(str::trim).next_tuple::<(_, _)>())
        .group_by(|&(_, name)| name)
        .into_iter()
        .map(|(name, itr)| {
            (
                name,
                itr.filter_map(|pair| {
                    let v = pair.0.splitn(2, "..").map(str::trim).collect_vec();
                    match &v[..] {
                        &[l, r] => Some((l, r)),
                        &[x] => Some((x, x)),
                        _ => None,
                    }
                }).collect_vec(),
            )
        })
        .collect();

    let merged_scripts: LinkedHashMap<&str, Vec<(String, String)>> = scripts
        .iter()
        .map(|(name, ranges)| {
            let ranges_in_other_scripts: Vec<(i32, i32)> = scripts
                .iter()
                .filter(|&(n, _)| n != name)
                .flat_map(|(_, r)| r)
                .map(|&(l, r)| {
                    (
                        i32::from_str_radix(l, 16).unwrap(),
                        i32::from_str_radix(r, 16).unwrap(),
                    )
                })
                .collect();

            let merged_ranges: Vec<(String, String)> = ranges
                .iter()
                .map(|&(l, r)| (l.to_string(), r.to_string()))
                .coalesce(|x, y| match ranges_in_other_scripts.iter().all(|&(left,
                   right)| {
                    right < i32::from_str_radix(&*x.0, 16).unwrap() ||
                        left > i32::from_str_radix(&*y.1, 16).unwrap()
                }) {
                    true => Ok((x.0, y.1)),
                    false => Err((x, y)),
                })
                .collect();

            (*name, merged_ranges)
        })
        .collect();

    let codes = merged_scripts
        .iter()
        .map(|(k, v)| {
            format!(
                "Range::Script {{ name: String::from(\"{}\"), code_points: vec![{}] }}",
                k,
                v.iter()
                    .map(|&(ref start, ref end)| format!("(0x{}, 0x{})", start, end))
                    .join(", ")
            )
        })
        .join(&format!(",\n{}", " ".repeat(4 * 3)));

    format!(
        "
macro_rules! unicode_scripts {{
    () => (
        vec![
            {}
        ];
    )
}}
",
        codes
    )
}


fn write(output: &mut File, codes: String) {
    output.write_all(codes.as_bytes()).unwrap();
}

fn read_lines(path: &str) -> Vec<String> {
    let f = File::open(path).unwrap();
    let reader = BufReader::new(&f);
    reader
        .lines()
        .map(|l| l.unwrap())
        .filter(|l| !l.starts_with("#") && !l.is_empty())
        .filter_map(|l| l.splitn(2, "#").map(str::to_string).next())
        .collect()
}
