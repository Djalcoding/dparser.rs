//! This crate implements a way of parsing configuration files and interpreting them as different datatypes.
//! You should be using the provided datastructure [`datastructure::ParsedData`] to retrieve
//! elements inside of your configuration file.
//!
//! # How to write configuration file
//!   - "!!" represents comments and may be inline.
//!   - Each field must be defined with a name and a value, seperated with a colon
//!   - Supported types are :
//!     - Strings
//!     - Any integer or floating point number
//!     - Colors (Represented with a name, an hexadecimal string or rgba values)
//!     - Booleans
//!     - Arrays
//!   - whitespaces are ignored
//!   - Strings must be surrounded by ""
//!   - Arrays fields are seperated by commas
//!   - Raw data can be extracted with ParsedData::as_raw(&self, key:&str)
//! # Example
//!
//! ```text
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
//!     bool1: yes
//!     bool2: y
//!     bool2: true
//!     bool3: 1 !! Note that this doesn't work for every number, only 1
//!     bool1: no
//!     bool2: n
//!     bool2: false
//!     bool3: 0
//!     !! Raw data can be used for enums and such
//!     raw_data: EnumVariant
//!     !! This is an array
//!     !! Arrays are typed
//!     string array: "Hello, ", "World"
//!     number array: 1, 1.618, 2, 2.718, 3, 3.142
//!     boolean array: true, false, yes, no
//!     color array: (0,255,0), transparent, #ffffff, red
//!     raw array: I, Will, Survive
//!```
//! # Rust Code Example
//! ```rust, no_run
//! use std::path::PathBuf;
//! use djal_parser::datastructure::ParsedData;
//!     
//! let path = PathBuf::from("./src/example_config_file.dconfig");
//! let parsed_data = ParsedData::from_file(&path).unwrap();
//! assert_eq!(parsed_data.read_as::<f64>("number2").unwrap(), -12f64);
//! assert_eq!(parsed_data.read_as::<String>("text").unwrap(), String::from("Lorem Ipsum"));
//! assert_eq!(parsed_data.read_as::<String>("TeXt").unwrap(), String::from("Lorem Ipsum"));
//! ```

use std::result;

use crate::errors::ParsingResult;
pub mod color;
pub mod errors;
pub mod parse;
mod read;

/// A [`ParsingResult`] or a T
pub type Result<T> = result::Result<T, ParsingResult>;
pub mod datastructure {
    //! This module implements a data structure to easily retrieve and interpret data from a parsed
    //! text file
    use crate::{
        Result,
        errors::ParsingResult,
        parse::{ConfigConstructible, ConfigEntry, parse_file, split_on_commas},
    };
    use std::{
        collections::{HashMap, hash_map::Iter},
        path::Path,
    };

    type MapKey = String;
    type MapValue = (usize, String);
    /// Data structure used to retrive and interpret data from a parsed text file
    /// HashMap Wrapper
    #[derive(Debug)]
    pub struct ParsedData<'p> {
        file: &'p Path,
        map: HashMap<MapKey, MapValue>,
    }

    impl<'p> ParsedData<'p> {
        /// Create a new ParsedData from a filepath (`path`) provided as a String.
        /// This will result in an error if the file cannot be properly parsed.
        pub fn from_file(path: &'p Path) -> Result<Self> {
            let entries: Vec<ConfigEntry> = parse_file(path)?;
            Ok(ParsedData::from_entries(path, entries))
        }

        fn standardize_key(key: &str) -> String {
            key.chars()
                .filter(|c| c.is_ascii() && !c.is_whitespace())
                .collect::<String>()
                .to_lowercase()
        }

        /// Returns the raw data from a key inside of the text file, along with its line number.
        /// This can also return a [`ParsingResult`] if the key isn't present in the file or if the
        /// default value is explicitly requested.
        pub fn as_raw(&self, key: &str) -> Result<(usize, &str)> {
            let entry = self.map.get(&Self::standardize_key(key));
            if entry.is_none() {
                return Err(ParsingResult::NonExistantEntry(key.to_owned()));
            }
            let (line_number, string) = entry.as_ref().unwrap();
            if string == "default" {
                return Err(ParsingResult::DefaultValueRequested);
            }
            Ok((*line_number, string))
        }

        /// Try to construct [`T`] from the entry named [`key`] 
        /// This can also return a [`ParsingResult`] if the key isn't present in the file.
        /// [`See Also`]
        /// [`Self::read_as()`]
        pub fn read_as<T: ConfigConstructible>(&self, key: &str) -> Result<T> {
            let (line, raw) = self.as_raw(key)?;
            let result = T::parse(raw);
            if let Err(error) = result {
                Err(ParsingResult::ParsingError {
                    filepath: self.file.to_owned(),
                    line,
                    error,
                })
            } else {
                Ok(result.unwrap())
            }
        }
        /// Returns data interpreted as a raw array.
        /// This can also return a [`ParsingResult`] if the key isn't present in the file or if the
        /// default value was explicitly requested.
        /// This is the array-variant for [`ParsedData::as_raw()`]
        pub fn as_raw_array(&self, key: &str) -> Result<(usize, Vec<&str>)> {
            let (line, raw_text) = self.as_raw(key)?;
            let splitted = split_on_commas(raw_text);
            Ok((line, splitted))
        }
        /// Try to construct a [`Vec<T>`] from the entry named [`key`] 
        /// This can also return a [`ParsingResult`] if the key isn't present in the file.
        /// [`See Also`]
        /// [`Self::as_raw_array()`]
        pub fn read_as_array<T: ConfigConstructible>(&self, key: &str) -> Result<Vec<T>> {
            let (line, raw_array) = self.as_raw_array(key)?;
            raw_array.iter().map(|raw_data| match T::parse(raw_data) {
                Ok(data) => Ok(data),
                Err(error) => Err(ParsingResult::ParsingError {
                    filepath: self.file.to_owned(),
                    line,
                    error,
                }.array_error()),
            }).collect()
        }

        /// See [`HashMap::iter()`]
        pub fn iter(&self) -> Iter<'_, MapKey, MapValue> {
            self.map.iter()
        }
        fn from_entries(path: &'p Path, entries: Vec<ConfigEntry>) -> Self {
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
