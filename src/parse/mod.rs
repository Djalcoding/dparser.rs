use std::{fmt::Display, io::Error, panic};

use crate::read::read_file;


pub struct Entry {
    name: String,
    content: String,
}

impl Entry {
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn content(&self) -> &String {
        &self.content
    }
}

impl Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Key : {} \nContent : {}", self.name(), self.content())
    }
}

pub fn parse_file(path: &String) -> Result<Vec<Entry>, Error>{
    let block = read_file(path)?;
    let mut entries: Vec<Entry> = Vec::new();
    for line in block.lines() {
        let trimmed_line: String = line.chars().filter(|c| !c.is_whitespace()).collect();
        if trimmed_line.starts_with("?") {
            continue;
        }

        let mut name: String = String::new();
        let mut value: String = String::new();

        for element in trimmed_line.split(':') {
            if name.is_empty() {
                name = String::from(element);
                continue;
            } else if value.is_empty() {
                value = String::from(element);
                continue;
            } else {
                break;
            }
        }

        entries.push(Entry {
            name,
            content: value,
        });
    }

    Ok(entries)
}
