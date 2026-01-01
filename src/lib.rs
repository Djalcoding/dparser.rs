//! This crate implements a way of parsing text files and interpreting them
//! as different datatypes.
pub mod error_handling;
pub mod parse;
mod read;

pub mod datastructure {

    //! This module implements a data structure to easily retrieve and interpret data from a parsed
    //! text file
    use std::{collections::HashMap, path::Path};

    use crate::{
        color::Color,
        error_handling::{FileReadingError, ParsingError},
        parse::{Entry, parse_file},
    };
    /// This is the data structure used to retrive and interpret data from a parsed text file, this
    /// is implemented as a HashMap for constant time retrival
    pub struct ParsedData {
        file: String,
        map: HashMap<String, (usize, String)>,
    }

    impl ParsedData {
        /// ## Arguments
        ///
        /// Create a new ParsedData from a filepath (`path`) provided as a String.
        ///
        /// ## Return type
        ///
        /// Returns a Result<ParsedData, Error>.
        /// This will result in an error if the file cannot be properly parsed.
        pub fn from_file(path: &Path) -> Result<Self, FileReadingError> {
            let entries: Vec<Entry> = parse_file(&path.display().to_string())?;
            Ok(ParsedData::from_entries(
                &path.display().to_string(),
                entries,
            ))
        }

        fn from_entries(path: &str, entries: Vec<Entry>) -> Self {
            let mut map: HashMap<String, (usize, String)> = HashMap::new();

            for entry in entries {
                map.insert(
                    String::from(entry.name()),
                    (entry.line_number(), String::from(entry.content())),
                );
            }

            Self {
                file: path.to_string(),
                map,
            }
        }

        pub fn as_raw(&self, key: &str) -> Result<(usize, String), FileReadingError> {
            let modified_key = key
                .chars()
                .filter(|c| c.is_alphanumeric())
                .collect::<String>()
                .to_lowercase();

            let entry = self.map.get(&modified_key);
            if entry.is_none() {
                return Err(FileReadingError::new(
                    self.file.to_string(),
                    format!("key '{key}' could not be found inside of the file"),
                ));
            }
            Ok(entry.unwrap().clone())
        }

        pub fn as_text(&self, key: &str) -> Result<String, FileReadingError> {
            let (line, raw) = self.as_raw(key)?;
            let mut in_text: bool = false;
            let mut quote_count: u16 = 0;
            let s: String = raw
                .chars()
                .filter(|c| {
                    if *c == '"' {
                        quote_count += 1;
                        in_text = !in_text;
                        return false;
                    }
                    in_text
                })
                .collect();
            if quote_count > 2 {
                return Err(FileReadingError::from(ParsingError::new(
                    self.file.to_string(),
                    format!("Too many double quotes are present on key {key}"),
                    line,
                )));
            } else if s.is_empty() {
                return Err(FileReadingError::from(ParsingError::new(
                    self.file.to_string(),
                    format!("no value was specified for key {key} (try inserting quotes)"),
                    line,
                )));
            }
            Ok(s)
        }

        pub fn as_number(&self, key: &str) -> Result<f64, FileReadingError> {
            let (line, raw) = self.as_raw(key)?;
            Ok(f64_from_string(self.file.to_string(), line, &raw)?)
        }

        pub fn as_boolean(&self, key: &str) -> Result<bool, FileReadingError> {
            let (line, raw) = self.as_raw(key)?;
            let lower_raw = raw.to_lowercase();

            let yes: bool =
                lower_raw == "yes" || lower_raw == "y" || lower_raw == "true" || lower_raw == "1";
            let no: bool =
                lower_raw == "no" || lower_raw == "n" || lower_raw == "false" || lower_raw == "0";

            if !no && !yes {
                return Err(FileReadingError::from(ParsingError::new(
                    self.file.to_string(),
                    format!("Invalid symbol '{raw}' was found"),
                    line,
                )));
            }
            Ok(yes)
        }

        pub fn as_color(&self, key: &str) -> Result<Color, FileReadingError> {
            let (line, raw) = self.as_raw(key)?;
            let rgb = Color::from_rgb_string(&raw);
            let hexadecimal = Color::from_hexadecimal(&raw);
            let text = Color::from_color_string(&raw);

            if let Ok(color) = rgb {
                return Ok(color);
            } else if let Ok(color) = hexadecimal {
                return Ok(color);
            } else if let Ok(color) = text {
                return Ok(color);
            }
            Err(FileReadingError::from(ParsingError::new(
                self.file.to_string(),
                format!("{raw} could not be interpreted as a color"),
                line,
            )))
        }
    }

    fn f64_from_string(file: String, line: usize, str: &str) -> Result<f64, ParsingError> {
        let mut number: f64 = 0.0;
        let mut decimal: f64 = 1.0;
        let mut decimals: bool = false;
        for character in str.chars() {
            if !character.is_numeric() && character != '.' {
                return Err(ParsingError::new(
                    file,
                    String::from("Non numeric character in f64"),
                    line,
                ));
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

        Ok(number)
    }
}

mod color {
    use hex::FromHexError;
    use std::{fmt::Display, io::Error};

    #[derive(Clone)]
    pub struct Color {
        r: u8,
        g: u8,
        b: u8,
        hexadecimal: String,
    }

    impl Color {
        pub fn rgb(r: u8, g: u8, b: u8) -> Self {
            Color {
                r,
                g,
                b,
                hexadecimal: format!("#{r:02x}{g:02x}{b:02x}"),
            }
        }

        pub fn from_color_string(string: &String) -> Result<Self, Error> {
            match string.to_lowercase().as_str() {
                "red" => Ok(Color::rgb(255, 0, 0)),
                "green" => Ok(Color::rgb(0, 255, 0)),
                "blue" => Ok(Color::rgb(0, 0, 255)),
                "white" => Ok(Color::rgb(255, 255, 255)),
                "black" => Ok(Color::rgb(0, 0, 0)),
                "yellow" => Ok(Color::rgb(255, 255, 0)),
                "purple" => Ok(Color::rgb(255, 0, 255)),
                "cyan" => Ok(Color::rgb(0, 255, 255)),
                _ => Err(Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("'{string}' is not a recognized color"),
                )),
            }
        }

        pub fn from_rgb_string(string: &str) -> Result<Self, Error> {
            let mut decode: Vec<u8> = Vec::new();
            for color in string
                .split(|c: char| !c.is_numeric())
                .filter(|s| !s.is_empty())
            {
                let mut num: u16 = 0;
                for character in color.chars() {
                    num *= 10;
                    num += character as u16 - '0' as u16;
                    if num > 255 {
                        return Err(Error::new(
                            std::io::ErrorKind::InvalidInput,
                            format!("Value over 255 in rgb : {num} at index {}", decode.len()),
                        ));
                    }
                }
                decode.push(num as u8);
            }
            if decode.len() < 3 {
                return Err(Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "less than 3 arguments for color definition",
                ));
            }
            Ok(Color::rgb(decode[0], decode[1], decode[2]))
        }

        pub fn from_hexadecimal(hexadecimal_color: &String) -> Result<Self, FromHexError> {
            let mut color: String = hexadecimal_color.to_string();
            if hexadecimal_color.starts_with("#") {
                color = hexadecimal_color.chars().filter(|c| *c != '#').collect();
            }
            let decode = hex::decode(&color)?;

            Ok(Color::rgb(decode[0], decode[1], decode[2]))
        }

        pub fn red(&self) -> u8 {
            self.r
        }

        pub fn green(&self) -> u8 {
            self.g
        }

        pub fn blue(&self) -> u8 {
            self.b
        }

        pub fn hexadecimal_value(&self) -> &String {
            &self.hexadecimal
        }
    }

    impl Display for Color {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "r: {}, g: {}, b: {}, hex: {}",
                self.red(),
                self.green(),
                self.blue(),
                self.hexadecimal_value()
            )
        }
    }
}
