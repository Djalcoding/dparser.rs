use std::{fmt::Display, path::Path};

use crate::{
    color::Color,
    error_handling::{FileReadingError, ParsingError},
    read::read_file,
};
pub struct Entry {
    name: String,
    content: String,
    line: usize,
}

impl Entry {
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn content(&self) -> &String {
        &self.content
    }

    pub fn line_number(&self) -> usize {
        self.line
    }
}

impl Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Key : {} \nContent : {}", self.name(), self.content())
    }
}

pub(crate) fn parse_file(path: &'_ Path) -> Result<Vec<Entry>, FileReadingError<'_>> {
    let block_result = read_file(path);
    let mut block = String::new();
    if let Ok(b) = block_result {
        block = b;
    } else if let Err(e) = block_result {
        return Err(FileReadingError::new(Path::new(path), e.to_string()));
    }

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
                return Err(FileReadingError::from_parsing_error(
                    Path::new(path),
                    ParsingError::new(String::from("Too many Objects on single line"), line_number),
                ));
            }
        }
        if !name.is_empty() {
            entries.push(Entry {
                name: name.to_lowercase(),
                content: value,
                line: line_number + 1,
            });
        }
    }

    Ok(entries)
}

pub(crate) fn split_on_commas(array: &str) -> Vec<&str> {
    let mut output: Vec<&str> = vec![];
    let mut left = 0;
    let mut in_text = false;
    let mut parenthesis_depth: i32 = 0;
    for (right, c) in array.char_indices() {
        match c {
            '"' => in_text = !in_text,
            '(' => parenthesis_depth += 1,
            ')' => parenthesis_depth -= 1,
            ',' if !in_text && parenthesis_depth == 0 => {
                output.push(&array[left..right]);
                left = right + 1;
            }
            _ => {}
        }
    }
    output.push(&array[left..array.len()]);
    output
}

pub(crate) fn parse_color(raw: &str) -> Result<Color, String> {
    let rgb = Color::from_rgb_string(raw);
    let hexadecimal = Color::from_hexadecimal(raw);
    let text = Color::from_color_string(raw);

    if let Ok(color) = rgb {
        return Ok(color);
    } else if let Ok(color) = hexadecimal {
        return Ok(color);
    } else if let Ok(color) = text {
        return Ok(color);
    }
    Err(format!("{raw} could not be interpreted as a color"))
}

pub(crate) fn parse_text(raw: &str) -> Result<String, String> {
    let mut quote_count: u16 = 0;
    let mut first_quote: usize = 0xFFFFFFFF;
    let mut last_quote: usize = 0x0;

    for (i, c) in raw.chars().enumerate() {
        if c != '"' {
            continue;
        }
        quote_count += 1;
        first_quote = std::cmp::min(first_quote, i);
        last_quote = std::cmp::max(last_quote, i);
    }
    let trimmed: String = raw
        .chars()
        .enumerate()
        .filter(|(i, _)| *i > first_quote && *i < last_quote)
        .map(|(_, c)| c)
        .collect();

    if quote_count < 2 {
        return Err(format!("There isn't enough double quotes in '{raw}'"));
    }
    Ok(trimmed)
}
pub(crate) fn parse_f64(str: &str) -> Result<f64, &str> {
    let mut number: f64 = 0.0;
    let mut decimal: f64 = 1.0;
    let mut decimals: bool = false;
    let mut negative: bool = false;
    for character in str.chars() {
        if !character.is_numeric() && character != '.' && character != '-' {
            return Err("Non numeric character in f64");
        }
        if character == '-' {
            negative = true;
            continue;
        }
        if !decimals {
            if character == '.' {
                decimals = true;
                continue;
            }
        } else {
            decimal /= 10.0
        }
        let num = character as u32 - '0' as u32;
        if decimal == 1.0 {
            number *= 10.0;
        }
        number += num as f64 * decimal;
    }
    if negative {
        return Ok(-number);
    }
    Ok(number)
}

#[cfg(test)]
mod tests {
    use crate::parse::split_on_commas;

    #[test]
    fn test_split_on_commas() {
        let out = split_on_commas("1,2,3,4,5");
        assert_eq!(out, vec!["1", "2", "3", "4", "5"]);
        let out = split_on_commas("\"I, love\",\"The\",\"World\"");
        assert_eq!(out, vec!["\"I, love\"", "\"The\"", "\"World\""]);
    }
}
