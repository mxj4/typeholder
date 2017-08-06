use range::Range;


#[derive(Debug)]
pub struct Family {
    pub name: String,
    pub stripped_ranges: Vec<Range>,
}
