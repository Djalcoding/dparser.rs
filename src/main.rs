use std::path::Path;

use djal_parser::{datastructure::ParsedData};

fn main() {
    let path =
        Path::new("/home/bert/Projects/RustProjects/djal_parser/src/example_config_file.dconfig");
    let s = ParsedData::from_file(path);
    let Ok(s) = s else {panic!("{:?}", s.unwrap_err());};
    println!("{:?}", s.as_text_array("string array").unwrap());
    println!("{:?}", s.as_boolean_array("boolean array").unwrap());
    println!("{:?}", s.as_color_array("color array").unwrap());
    println!("{:?}", s.as_number_array("number array").unwrap());
    println!("{:?}", s.as_raw_array("raw array").unwrap());
}
