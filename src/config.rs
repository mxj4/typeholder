use alias::Alias;
use itertools::Itertools;
use family::Family;
use range::Range;
use sxd_document::Package;

use std::cell::RefCell;


#[derive(Debug)]
pub struct Config<'a> {
    pub scan_matches: Vec<&'a RefCell<Family>>,
    pub aliases: Vec<Alias<'a>>,
    pub residue: Package,
}

/* todo
impl Config<'a> {
    fn add_family(&self) {

    }
}
*/


#[test]
fn test_add_family() {}
