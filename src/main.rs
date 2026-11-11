use std::path::Path;

use djal_parser::{color::Color, datastructure::ParsedData};

fn main() {
    let path =
        Path::new("/home/bert/Projects/RustProjects/djal_parser/src/example_config_file.dconfig");
    let s = ParsedData::from_file(path);
    let Ok(s) = s else {panic!("{:?}", s.unwrap_err());};
    println!("{:?}", s.read_as_array::<String>("string array").unwrap());
    println!("{:?}", s.read_as_array::<bool>("boolean array").unwrap());
    println!("{:?}", s.read_as_array::<Color>("color array").unwrap());
    println!("{:?}", s.read_as_array::<f32>("number array").unwrap_or(vec![2.0,3.0]));
    println!("{:?}", s.as_raw_array("raw array").unwrap());
}
