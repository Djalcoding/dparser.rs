
use std::{
    fs::File, io::{Read, Result}, path::Path,
};


pub fn read_file(path: &Path) -> Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

