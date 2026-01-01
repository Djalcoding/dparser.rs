//! This crate implements a way of parsing configuration files and interpreting them as different datatypes.
//! You should be using the provided datastructure [`datastructure::ParsedData`] to retrieve
//! elements inside of your configuration file.
//!
//! # How to write configuration file
//!   - "!!" represents comments and may be inline.
//!   - Each field must be defined with a name and a value, seperated with a colon
//!   - Supported types are :
//!     - Strings
//!     - 64 bit precision floating point numbers
//!     - Colors (Represented with a name, an hexadecimal string or rgb values)
//!     - Booleans 
//! # Example
//!     !! This is documentation. 
//!         !! This is a number
//!     number1: 12.643
//!         !! This is also a number
//!     number2: -12
//!         !! This is a color as an rgb string
//!     color1: (0,255,0)
//!         !! This is a color as an hexadecimal color
//!     color2: #FFFFFF
//!         !! This is a color represented with a name
//!     color3: red
//!         !! This is text
//!     text: "Lorem Ipsum"
//!         !! Those is are booleans
//!     bool1 = yes
//!     bool2 = y
//!     bool2 = true
//!     bool3 = 1 !! Note that this doesn't work for every number, only 1
//!     bool1 = no
//!     bool2 = n
//!     bool2 = false
//!     bool3 = 0
//! # Rust Code Example
//! ```
//! let path = Path::new("djal_parser/src/example_config_file.dconfig");
//! let parsed_data_map = ParsedData::from_file(path).unwrap();
//! assert_eq(parsedDataMap.as_text("Hello, World !"), Ok("Hello, World !"))  
//! ```
pub mod color;
pub mod error_handling;
mod parse;
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

        /// Returns the raw data from a key inside of the text file, along with its line number
        /// This can also return a FileReadingError if the key isn't present in the file.
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

        /// Returns data interpreted as a piece of text from a key inside of the text file.
        /// This can also return a FileReadingError if the key isn't present in the file, too many
        /// quotes are present or if the value if empty.
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
                    format!("Too many double quotes are present value of key '{key}'"),
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

        /// Returns data interpreted as f64 from a key inside of the text file.
        /// This can also return a FileReadingError if the key isn't present in the file or if
        /// non-numeric character is inside of the value
        pub fn as_number(&self, key: &str) -> Result<f64, FileReadingError> {
            let (line, raw) = self.as_raw(key)?;
            Ok(f64_from_string(self.file.to_string(), line, &raw)?)
        }

        /// Returns data interpreted as boolean from a key inside of the text file.
        /// This can also return a FileReadingError if the key isn't present in the file or if
        /// an unrecognized symbol is the value.
        /// ## Note
        /// the recognized symbols are :
        ///   - 'yes', 'y', 'true' and '1' for true,
        ///   - 'no', 'n', 'false' and '0' for false.
        /// They are case insensitive
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

        /// Returns data interpreted as Color from a key inside of the text file.
        /// This can also return a FileReadingError if the key isn't present in the file or if
        /// the raw data cannot be parsed as a color as an rgb value, an hexadecimal or a color
        /// name
        /// [See Also]
        /// [`Color`] for more information
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
    }

    fn f64_from_string(file: String, line: usize, str: &str) -> Result<f64, ParsingError> {
        let mut number: f64 = 0.0;
        let mut decimal: f64 = 1.0;
        let mut decimals: bool = false;
        let mut negative:bool = false;
        for character in str.chars() {
            if !character.is_numeric() && character != '.' && character != '-'{
                return Err(ParsingError::new(
                    file,
                    String::from("Non numeric character in f64"),
                    line,
                ));
            }
            if(character == '-'){
                negative =true;
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
        if negative{
            return Ok(-number);
        }
        Ok(number)
    }
}
