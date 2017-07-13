include!(concat!(env!("OUT_DIR"), "/ucd.rs"));

#[derive(Debug)]
pub enum Range {
    Block {
        name: String,
        code_points: (i32, i32),
    },
    Script {
        name: String,
        code_points: Vec<(i32, i32)>,
    },
    Custom {
        name: String,
        code_points: (i32, i32),
    },
}

lazy_static! {
    pub static ref UNICODE_BLOCKS: Vec<Range> = unicode_blocks!();
    pub static ref UNICODE_SCRIPTS: Vec<Range> = unicode_scripts!();
}
