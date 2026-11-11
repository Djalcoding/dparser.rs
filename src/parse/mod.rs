use std::{error::Error, fmt::Display, path::Path};

use crate::{
    color::Color,
    errors::{BoolParsingError, NumberParsingError, ParsingResult, TextParsingError},
    read::read_file,
};
pub(crate) struct ConfigEntry {
    name: String,
    content: String,
    line: usize,
}

impl ConfigEntry {
    pub(crate) fn name(&self) -> &String {
        &self.name
    }

    pub(crate) fn content(&self) -> &String {
        &self.content
    }

    pub(crate) fn line_number(&self) -> usize {
        self.line
    }
}

impl Display for ConfigEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Key : {} \nContent : {}", self.name(), self.content())
    }
}

pub(crate) fn parse_file(path: &'_ Path) -> Result<Vec<ConfigEntry>, ParsingResult> {
    let block_result = read_file(path);
    let mut block = String::new();
    if let Ok(b) = block_result {
        block = b;
    } else if let Err(e) = block_result {
        return Err(ParsingResult::IOError {
            filepath: path.to_owned(),
            error: e,
        });
    }

    let mut entries: Vec<ConfigEntry> = Vec::new();
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
        let (mut without_comment, _) = trimmed_line.split_once("!!").unwrap_or((&trimmed_line, ""));
        without_comment = without_comment.trim();
        if without_comment.is_empty() {
            continue;
        }
        let separator_index = without_comment.find(":");
        if separator_index.is_none() {
            return Err(ParsingResult::NoSeperator {
                filepath: path.to_owned(),
                line: line_number,
            });
        }
        let (name, contents) = without_comment.split_at(without_comment.find(":").unwrap());

        if !name.is_empty() {
            entries.push(ConfigEntry {
                name: name.to_lowercase(),
                content: contents[1..].to_owned(),
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

/// Represents a type that can be constructed from a config entry
pub trait ConfigConstructible: Sized {
    fn parse(raw_data: &str) -> Result<Self, Box<dyn Error>>;
}

impl ConfigConstructible for bool {
    fn parse(raw_data: &str) -> Result<Self, Box<dyn Error>> {
        let lower_raw = raw_data.to_lowercase();

        let yes: bool =
            lower_raw == "yes" || lower_raw == "y" || lower_raw == "true" || lower_raw == "1";
        let no: bool =
            lower_raw == "no" || lower_raw == "n" || lower_raw == "false" || lower_raw == "0";

        if !no && !yes {
            return Err(Box::new(BoolParsingError::UnknownName(raw_data.to_owned())));
        }
        Ok(yes)
    }
}

impl ConfigConstructible for String {
    fn parse(raw_data: &str) -> Result<Self, Box<dyn Error>> {
        let mut quote_count: u16 = 0;
        let mut first_quote: usize = 0xFFFFFFFF;
        let mut last_quote: usize = 0x0;

        for (i, c) in raw_data.chars().enumerate() {
            if c != '"' {
                continue;
            }
            quote_count += 1;
            first_quote = std::cmp::min(first_quote, i);
            last_quote = std::cmp::max(last_quote, i);
        }
        let trimmed: String = raw_data
            .chars()
            .enumerate()
            .filter(|(i, _)| *i > first_quote && *i < last_quote)
            .map(|(_, c)| c)
            .collect();

        if quote_count < 2 {
            return Err(Box::from(TextParsingError::NotEnoughDoubleQuotes(
                raw_data.to_owned(),
            )));
        }
        Ok(trimmed)
    }
}

impl ConfigConstructible for Color {
    fn parse(raw_data: &str) -> Result<Self, Box<dyn Error>> {
        let rgb = Color::from_rgb_string(raw_data);
        if let Ok(color) = rgb {
            return Ok(color);
        }
        let hexadecimal = Color::from_hexadecimal(raw_data);
        if let Ok(color) = hexadecimal {
            return Ok(color);
        }
        let text = Color::from_color_string(raw_data);
        if let Ok(color) = text {
            return Ok(color);
        }

        Err(Box::new(rgb.unwrap_err()))
    }
}

macro_rules! impl_for_float {
    ($($type:ty),*) => {
        $(
impl ConfigConstructible for $type {
    fn parse(raw_data: &str) -> Result<Self, Box<dyn Error>> {
        match raw_data.parse::<$type>() {
            Ok(number) => {
                if !number.is_finite() {
                    Err(Box::from(NumberParsingError::TooLarge))
                } else {
                    Ok(number)
                }
            }
            Err(error) => Err(Box::from(error)),
        }
    }
}
        )*
    };
}

macro_rules! impl_for_integer {
    ($($type:ty),*) => {
        $(
impl ConfigConstructible for $type {
    fn parse(raw_data: &str) -> Result<Self, Box<dyn Error>> {
        Ok(raw_data.parse::<$type>()?)
    }
}
        )*
    };
}

impl_for_float!(f64, f32);
impl_for_integer!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);

#[cfg(test)]
mod tests {
    use crate::parse::{ConfigConstructible, split_on_commas};

    #[test]
    fn test_split_on_commas() {
        let out = split_on_commas("1,2,3,4,5");
        assert_eq!(out, vec!["1", "2", "3", "4", "5"]);
        let out = split_on_commas("\"I, love\",\"The\",\"World\"");
        assert_eq!(out, vec!["\"I, love\"", "\"The\"", "\"World\""]);
    }
    #[test]
    fn test_parse_number() {
        let output = f64::parse("123");
        assert_eq!(output.unwrap(), 123f64);
        let output = f64::parse("123.2302");
        assert_eq!(output.unwrap(), 123.2302f64);
        let output = u128::parse("456");
        assert_eq!(output.unwrap(), 456u128);
        let output = u8::parse("255");
        assert_eq!(output.unwrap(), 255u8);
        let overflow = u8::parse("256");
        assert!(overflow.is_err());
    }
}
