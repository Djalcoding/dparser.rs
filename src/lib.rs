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
//!     - Colors (Represented with a name, an hexadecimal string or rgba values)
//!     - Booleans
//!   - whitespaces are ignored
//!   - Strings must be surrounded by ""
//!   - Raw data can be extracted with ParsedData::as_raw(&self, key:&str)
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
//! ```rust
//! fn demonstrate_parsed_data {
//!     let path = PathBuf::from("./src/example_config_file.dconfig");
//!     let parsed_data_map = ParsedData::from_file(&path).unwrap();
//!     assert_eq!(parsedDataMap.as_number("number2"), Ok(-12 as f64))  
//!     assert_eq!(parsedDataMap.as_text("text"), Ok("Lorem Ipsum"))  
//!     assert_eq!(parsedDataMap.as_text("TeXt"), Ok("Lorem Ipsum"))  
//! }
//! ```
pub mod color;
pub mod error_handling;
mod parse;
mod read;

pub mod datastructure {

    //! This module implements a data structure to easily retrieve and interpret data from a parsed
    //! text file
    use crate::{
        color::Color,
        error_handling::{FileReadingError, ParsingError},
        parse::{Entry, parse_color, parse_f64, parse_file, parse_text, split_on_commas},
    };
    use std::{
        collections::{HashMap, hash_map::Iter},
        path::Path,
    };

    type MapKey = String;
    type MapValue = (usize, String);
    /// This is the data structure used to retrive and interpret data from a parsed text file, this
    /// is implemented as a HashMap for constant time retrival
    pub struct ParsedData<'p> {
        file: &'p Path,
        map: HashMap<MapKey, MapValue>,
    }

    impl<'p> ParsedData<'p> {
        /// ## Arguments
        ///
        /// Create a new ParsedData from a filepath (`path`) provided as a String.
        ///
        /// ## Return type
        ///
        /// Returns a Result<ParsedData, Error>.
        /// This will result in an error if the file cannot be properly parsed.
        pub fn from_file(path: &'p Path) -> Result<Self, FileReadingError<'p>> {
            let entries: Vec<Entry> = parse_file(path)?;
            Ok(ParsedData::from_entries(path, entries))
        }

        /// Returns the raw data from a key inside of the text file, along with its line number
        /// This can also return a FileReadingError if the key isn't present in the file.
        fn standardize_key(key: &str) -> String {
            key.chars()
                .filter(|c| c.is_ascii() && !c.is_whitespace())
                .collect::<String>()
                .to_lowercase()
        }

        pub fn as_raw(&self, key: &str) -> Result<(usize, &str), FileReadingError<'_>> {
            let entry = self.map.get(&Self::standardize_key(key));
            if entry.is_none() {
                return Err(FileReadingError::new(
                    self.file,
                    format!("key '{key}' could not be found inside of the file"),
                ));
            }
            let (line, string) = entry.as_ref().unwrap();
            Ok((*line, string))
        }

        /// Returns data interpreted as a piece of text from a key inside of the text file.
        /// This can also return a FileReadingError if the key isn't present in the file, too many
        /// quotes are present or if the value if empty.
        pub fn as_text(&self, key: &str) -> Result<String, FileReadingError<'_>> {
            let (line, raw) = self.as_raw(key)?;
            let parsed = parse_text(raw);
            match parsed {
                Ok(output) => Ok(output),
                Err(error_message) => Err(FileReadingError::from_parsing_error(
                    self.file,
                    ParsingError::new(error_message, line),
                )),
            }
        }

        /// Returns data interpreted as f64 from a key inside of the text file.
        /// This can also return a FileReadingError if the key isn't present in the file or if
        /// non-numeric character is inside of the value
        pub fn as_number(&self, key: &str) -> Result<f64, FileReadingError<'_>> {
            let (line, raw) = self.as_raw(key)?;
            let result = parse_f64(raw);
            if let Ok(value) = result {
                Ok(value)
            } else {
                Err(FileReadingError::from_parsing_error(
                    self.file,
                    ParsingError::new(String::from(result.unwrap_err()), line),
                ))
            }
        }

        /// Returns data interpreted as boolean from a key inside of the text file.
        /// This can also return a FileReadingError if the key isn't present in the file or if
        /// an unrecognized symbol is the value.
        /// ## Note
        /// the recognized symbols are :
        ///   - 'yes', 'y', 'true' and '1' for true,
        ///   - 'no', 'n', 'false' and '0' for false.
        ///
        /// They are case insensitive
        pub fn as_boolean(&self, key: &str) -> Result<bool, FileReadingError<'_>> {
            let (line, raw) = self.as_raw(key)?;
            let lower_raw = raw.to_lowercase();

            let yes: bool =
                lower_raw == "yes" || lower_raw == "y" || lower_raw == "true" || lower_raw == "1";
            let no: bool =
                lower_raw == "no" || lower_raw == "n" || lower_raw == "false" || lower_raw == "0";

            if !no && !yes {
                return Err(FileReadingError::from_parsing_error(
                    self.file,
                    ParsingError::new(format!("Invalid symbol '{raw}' was found"), line),
                ));
            }
            Ok(yes)
        }

        /// Returns data interpreted as Color from a key inside of the text file.
        /// This can also return a FileReadingError if the key isn't present in the file or if
        /// the raw data cannot be parsed as a color as an rgb value, an hexadecimal or a color
        /// name
        /// [See Also]
        /// [`Color`] for more information
        pub fn as_color(&self, key: &str) -> Result<Color, FileReadingError<'_>> {
            let (line, raw) = self.as_raw(key)?;
            let color = parse_color(raw);
            match color {
                Ok(color) => Ok(color),
                Err(error_message) => Err(FileReadingError::from_parsing_error(
                    self.file,
                    ParsingError::new(error_message, line),
                )),
            }
        }

        pub fn as_raw_array(&self, key: &str) -> Result<(usize, Vec<&str>), FileReadingError<'_>> {
            let (line, raw_text) = self.as_raw(key)?;
            let splitted = split_on_commas(raw_text);
            Ok((line, splitted))
        }

        pub fn as_text_array(&self, key: &str) -> Result<Vec<String>, FileReadingError<'_>> {
            let (line, raw) = self.as_raw_array(key)?;
            raw.iter()
                .map(|array_element| match parse_text(array_element) {
                    Ok(data) => Ok(data),
                    Err(error_message) => Err(FileReadingError::from_parsing_error(
                        self.file,
                        ParsingError::new(error_message, line),
                    )),
                })
                .collect()
        }

        pub fn as_color_array(&self, key: &str) -> Result<Vec<Color>, FileReadingError<'_>> {
            let (line, raw) = self.as_raw_array(key)?;
            raw.iter()
                .map(|array_element| match parse_color(array_element) {
                    Ok(valid_color) => Ok(valid_color),
                    Err(error_message) => Err(FileReadingError::from_parsing_error(
                        self.file,
                        ParsingError::new(error_message, line),
                    )),
                })
                .collect()
        }

        pub fn as_number_array(&self, key: &str) -> Result<Vec<f64>, FileReadingError<'_>> {
            let (line, raw) = self.as_raw_array(key)?;
            raw.iter()
                .map(|array_element| match parse_f64(array_element) {
                    Ok(valid_color) => Ok(valid_color),
                    Err(error_message) => Err(FileReadingError::from_parsing_error(
                        self.file,
                        ParsingError::new(String::from(error_message), line),
                    )),
                })
                .collect()
        }

        pub fn iter(&self) -> Iter<'_, MapKey, MapValue> {
            self.map.iter()
        }
        fn from_entries(path: &'p Path, entries: Vec<Entry>) -> Self {
            let mut map: HashMap<String, (usize, String)> = HashMap::new();

            for entry in entries {
                map.insert(
                    String::from(entry.name()),
                    (entry.line_number(), String::from(entry.content())),
                );
            }

            Self { file: path, map }
        }
    }
}
