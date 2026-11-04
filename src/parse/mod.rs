use std::{fmt::Display, path::Path};

use crate::{
    color::Color,
    error_handling::{
        BoolParsingError, ColorParsingError, NumberParsingError, ParsingError, TextParsingError,
    },
    read::read_file,
};
pub struct ConfigEntry {
    name: String,
    content: String,
    line: usize,
}

impl ConfigEntry {
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

impl Display for ConfigEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Key : {} \nContent : {}", self.name(), self.content())
    }
}

pub(crate) fn parse_file(path: &'_ Path) -> Result<Vec<ConfigEntry>, ParsingError<'_>> {
    let block_result = read_file(path);
    let mut block = String::new();
    if let Ok(b) = block_result {
        block = b;
    } else if let Err(e) = block_result {
        return Err(ParsingError::IOError {
            filepath: path,
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
        let separator_index = without_comment.find(":");
        if separator_index.is_none() {
            return Err(ParsingError::NoSeperator {
                filepath: path,
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

pub(crate) fn parse_color(raw: &str) -> Result<Color, ColorParsingError<'_>> {
    let rgb = Color::from_rgb_string(raw);
    let hexadecimal = Color::from_hexadecimal(raw);
    let text = Color::from_color_string(raw);

    if let Ok(color) = rgb {
        Ok(color)
    } else if let Ok(color) = hexadecimal {
        Ok(color)
    } else if let Ok(color) = text {
        Ok(color)
    } else {
        Err(rgb.unwrap_err())
    }
}

pub(crate) fn parse_text(raw: &str) -> Result<String, TextParsingError<'_>> {
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
        return Err(TextParsingError::NotEnoughDoubleQuotes(raw));
    }
    Ok(trimmed)
}

pub(crate) fn parse_number(str: &str) -> Result<f64, NumberParsingError<'_>> {
    let mut negative: bool = false;
    let mut number: f64 = 0.0;
    let mut decimal_start: Option<usize> = None;
    let mut found_first_digit: bool = false;
    for (idx, character) in str.chars().enumerate() {
        match character {
            '.' => {
                decimal_start = Some(idx);
            }
            '-' if !found_first_digit => negative = !negative,
            _ if !character.is_ascii_digit() => {
                return Err(NumberParsingError::NonNumericCharacter(str, character));
            }
            _ => {
                found_first_digit = true;
                let character_f64 = (character as u8 - b'0') as f64;
                if let Some(dot_index) = decimal_start {
                    number += character_f64 / 10f64.powf((idx - dot_index) as f64)
                } else {
                    number *= 10.0;
                    number += character_f64;
                }
                if !number.is_finite() {
                    // f64 overflows to inf
                    return Err(NumberParsingError::TooLarge(str));
                }
            }
        }
    }
    Ok(number)
}

pub(crate) fn parse_bool(raw: &str) -> Result<bool, BoolParsingError<'_>> {
    let lower_raw = raw.to_lowercase();

    let yes: bool =
        lower_raw == "yes" || lower_raw == "y" || lower_raw == "true" || lower_raw == "1";
    let no: bool =
        lower_raw == "no" || lower_raw == "n" || lower_raw == "false" || lower_raw == "0";

    if !no && !yes {
        return Err(BoolParsingError::UnknownName(raw));
    }
    Ok(yes)
}

#[cfg(test)]
mod tests {
    use crate::parse::{parse_number, split_on_commas};

    #[test]
    fn test_split_on_commas() {
        let out = split_on_commas("1,2,3,4,5");
        assert_eq!(out, vec!["1", "2", "3", "4", "5"]);
        let out = split_on_commas("\"I, love\",\"The\",\"World\"");
        assert_eq!(out, vec!["\"I, love\"", "\"The\"", "\"World\""]);
    }
    #[test]
    fn test_parse_number() {
        let output = parse_number("123");
        assert_eq!(output.unwrap(), 123f64);
    }
}
