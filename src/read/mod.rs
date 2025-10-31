
use std::{
    fs::File,
    io::{ErrorKind, Read, Result},
};

use crate::config::ConfigBlock;

pub fn get_file_contents(path: String) -> ConfigBlock {
    let file_contents = read_file(&path);
    let error = get_error_message(&path, &file_contents);
    if let Some(err) = error {
        return ConfigBlock::invalid(err);
    }
    ConfigBlock::new(file_contents.unwrap(),path)
}

pub fn read_file(path: &String) -> Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn get_error_message(arg: &String, result: &Result<String>) -> Option<String> {
    if let Err(err) = result {
        match err.kind() {
            ErrorKind::PermissionDenied => Some(String::from("Permission Denied")),
            ErrorKind::InvalidFilename => Some(format!("Filename '{arg}' is invalid")),
            ErrorKind::NotFound => Some(format!("File '{arg}' does not exist")),
            _ => Some(format!("Error : {}", err.kind())),
        }
    } else {
        None
    }
}
