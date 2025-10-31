use djal_parser::{parse::{map::DataMap, parse_file}, read::get_file_contents};

fn main() {
    let s= parse_file(String::from("/home/bert/Projects/RustProjects/djal_parser/src/test.txt"));

    let map = DataMap::build(s);
    println!("{}", map.hexadecimal_colors().get("color").unwrap())
    

    
}
