use std::io;

use djal_parser::{datastructure::ParsedData};

fn main() -> io::Result<()> {
    let path = String::from("/home/bert/Projects/RustProjects/djal_parser/src/test.txt");

    let s = ParsedData::from_file(&path)?;

    println!("{}", s.as_color(&String::from("color")).unwrap());
    Ok(())
}
