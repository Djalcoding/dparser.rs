use std::{fmt::Display, io::Error};

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

pub fn parse_file(path: &String) -> Result<Vec<Entry>, Error> {
    let block = read_file(path)?;
    let mut entries: Vec<Entry> = Vec::new();
    for (line_number, line) in block.lines().enumerate() {
        let mut in_text: bool = false;
        let trimmed_line: String = line
            .chars()
            .filter(|c| {
                if *c == '"' {
                    in_text = !in_text;
                }
                !c.is_whitespace() || in_text
            })
            .collect();
        let (without_comment, _) = trimmed_line.split_once("!!").unwrap_or((&trimmed_line, ""));
        let mut name: String = String::new();
        let mut value: String = String::new();

        for element in without_comment.split(':') {
            if name.is_empty() {
                name = String::from(element);
                continue;
            } else if value.is_empty() {
                value = String::from(element);
                continue;
            } else {
                return Err(Error::new(std::io::ErrorKind::InvalidInput, format!("Too many arguments on line {line_number}")))
            }
        }
        if !name.is_empty() {
            entries.push(Entry {
                name,
                content: value,
            });
        }
    }

    Ok(entries)
}
