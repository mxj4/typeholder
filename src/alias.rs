use family::Family;

use std::cell::RefCell;


#[derive(Debug)]
pub struct Alias<'a> {
    pub name: String,
    pub prefer_list: Vec<&'a RefCell<Family>>,
}

/* todo
impl Alias {

}
*/
