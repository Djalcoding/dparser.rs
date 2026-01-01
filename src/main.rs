use std::path::Path;

use djal_parser::{datastructure::ParsedData};

fn main() {
    let path = Path::new("/home/bert/Projects/RustProjects/djal_parser/src/test.txt");
    let s = ParsedData::from_file(path).unwrap();

    println!("{}", s.as_text("color").unwrap());
}
