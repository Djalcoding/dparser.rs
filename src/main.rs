use djal_parser::parse::parse_file;


fn main() {
    let s = parse_file(&String::from("/home/bert/Projects/RustProjects/djal_parser/src/test.txt")).unwrap();
    for v in s {
        println!("{v}");
    }
    

}
