use itertools::Itertools;
use range::Range;
use std::process::Command;
use std::collections::HashSet;

// https://redd.it/3ajrx5
macro_rules! iproduct_concat {
    ( $( $x:expr ),+; [ $( $y:expr ),+ ] ) => {
        iproduct_concat_impl!(($($x,)+); (); $($y,)+)
    };
}
macro_rules! iproduct_concat_impl {
    ($xs:tt; ($($ex:tt)*);) => { [$($ex)*] };

    (($($x:expr,)+); ($($ex:tt)*); $y:expr, $($tail:tt)*) => {
        iproduct_concat_impl!(($($x,)+); ($($ex)* $(concat!($x, $y),)+); $($tail)*)
    };
}


pub struct Family {
    pub name: String,
    pub stripped_ranges: Vec<Range>,
}


const WEIGHT_KEYWORDS: [&str; 25] =
    iproduct_concat!(
    "semi", "demi", "hemi", "extra", "";
    ["black", "bold", "heavy", "light", "thin"]
);

const MORE_WEIGHT_KEYWORDS: [&str; 3] = ["medium", "normal", "regular"];

// generate list of font families using `fc-list` command.
// if the font already defined a family name without font weight
// remove family names with font weight
pub fn read_available_families() -> Vec<Family> {
    let output = Command::new("fc-list")
        .arg(":")
        .arg("family")
        .output()
        .expect("failed to execute `fc-list` to get available families");

    assert!(output.status.success());

    // construct weight words set
    let weight_words: HashSet<&str> = WEIGHT_KEYWORDS
        .iter()
        .chain(MORE_WEIGHT_KEYWORDS.iter())
        .map(|&x| x)
        .collect();

    String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|s| {
            let names: Vec<&str> = s.split(',').collect();
            names
                .iter()
                .cloned()
                // TODO: cover cases like `Open Sans Hebrew,Open Sans Hebrew Extra Bold`
                .filter(|s| match s.rsplitn(2, ' ').next_tuple::<(_, _)>() {
                    Some((weight, name)) => {
                        let lowercase = &weight.to_lowercase();
                        !names.contains(&name) || !weight_words.contains(lowercase.as_str())
                    }
                    None => true,
                })
                .join(",")
        })
        .unique()
        .map(|s| {
            Family {
                name: s,
                stripped_ranges: vec![],
            }
        })
        .sorted_by(|a, b| Ord::cmp(&a.name, &b.name))
}
