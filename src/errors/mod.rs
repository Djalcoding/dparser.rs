//! Errors that might occur while trying to parse a file

use std::{error::Error, fmt::Display,path::PathBuf, result};

use crate::errors::ColorParsingError::InvalidArgumentCount;

/// Represents an error that might occur while parsing a file
/// Everything is owned, so a clone operation may be necessary
#[derive(Debug)]
pub enum ParsingResult {
    /// Not an error, the explicit request that the default value be used
    DefaultValueRequested,
    /// A syscall error, wrapper for [`std::io::Error`].
    /// Contains filepath.
    // This is not a struct because I love flat girls
    IOError {
        filepath: PathBuf,
        error: std::io::Error,
    },
    /// Occurs when a line has no seperator.
    /// Contains error location.
    NoSeperator { filepath: PathBuf, line: usize },
    /// An error that occurs when a key is not present in a [`crate::datastructure::ParsedData`].
    NonExistantEntry(String),
    /// An error that occurs when failing to parse a color.
    /// Contains error location.
    /// See [`ColorParsingError`]
    ParsingError {
        filepath: PathBuf,
        line: usize,
        error: Box<dyn Error>,
    },
    /// Wrapper for another Error to indicate that this error happends over an array
    ArrayError(Box<ParsingResult>), // Boohou heap allocation, it only happens on errors
}

impl ParsingResult {
    /// Wrap an error into an array error
    /// [See Also]
    /// [`ParsingError::ArrayError`]
    pub fn array_error(self) -> Self {
        match self {
            Self::ArrayError(_) => self,
            _ => Self::ArrayError(Box::new(self)),
        }
    }
    /// Returns true if [`self`]
    pub fn is_error(&self) -> bool {
        !matches!(self, Self::DefaultValueRequested)
    }
}

pub type Result<T> = result::Result<T, ParsingResult>;

impl Display for ParsingResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DefaultValueRequested => {
                write!(f, "default value is requested")
            }
            Self::NonExistantEntry(entry) => {
                write!(f, "{entry} is not present in file")
            }
            Self::IOError { filepath, error } => {
                write!(f, "Failed to read {} : {error}", filepath.display())
            }
            Self::NoSeperator { filepath, line } => {
                write!(
                    f,
                    "No seperator are one line {line} in '{}'",
                    filepath.display()
                )
            }
            Self::ArrayError(error) => {
                write!(f, "Failed to parse array : {error}")
            }
            Self::ParsingError {
                filepath,
                line,
                error,
            } => {
                write!(
                    f,
                    "Failed to parse entry within '{}' on line {line} : {error}",
                    filepath.display()
                )
            }
        }
    }
}

impl Error for ParsingResult {}

#[derive(Debug, PartialEq, Clone)]
/// An error that describes a failure in number parsing
pub enum NumberParsingError {
    /// Occurs when input is too large for the wanted numeric output type
    TooLarge,
}
impl Display for NumberParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NumberParsingError::TooLarge => {
                write!(f, "float litteral is too large to be parsed")
            }
        }
    }
}
impl Error for NumberParsingError {}

#[derive(Debug, PartialEq, Clone)]
/// An error that describes a failure in boolean parsing
pub enum BoolParsingError {
    /// An error that occurs when the input string is not a recognized boolean string
    UnknownName(String),
}
impl Display for BoolParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self::UnknownName(name) = self;
        write!(
            f,
            "boolean couldn't be parsed : '{name}' is not a valid boolean identifier"
        )
    }
}
impl Error for BoolParsingError {}

#[derive(Debug, PartialEq, Clone)]
/// An error that describes a failure in text parsing
pub enum TextParsingError {
    /// An error that occurs when a the input string doesn't have atleast 2 double quotes
    NotEnoughDoubleQuotes(String),
}
impl Display for TextParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self::NotEnoughDoubleQuotes(name) = self;
        write!(
            f,
            "string couldn't be parsed : '{name}' doesn't have enough double quotes to be parsed"
        )
    }
}
impl Error for TextParsingError {}

#[derive(Debug, PartialEq, Clone)]
/// An error that describes a failure in color parsing
pub enum ColorParsingError {
    /// An error that occurs when there is either not enough or too much rgba arguments provided in
    /// the input string
    /// ```text
    /// !! This won't cause an error
    /// entry: (1,2,3)
    /// entry2: (1,2,3,4)
    /// !! This will
    /// entry3: (1,2)
    /// entry4: (1,2,3,4,5)
    /// ```
    InvalidArgumentCount { found: usize },
    /// An error that occurs when the input color string (red,white,transparent) is not recognized
    UnknownName(String),
    /// Occurs when any character in the input string fails [`char::is_ascii_digit()`] and isn't '('
    /// or ')'
    NonNumericCharacter(String, char),
    /// Occurs when any rgba argument is over 255
    Over255,
}

impl Display for ColorParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InvalidArgumentCount { found } => write!(
                f,
                "color couldn't be parsed : '{found}' arguments provided for color when 3 or 4 are expected"
            ),
            Self::UnknownName(name) => write!(f, "{name} is not a recognized color"),
            Self::NonNumericCharacter(raw, c) => {
                write!(
                    f,
                    "color couldn't be parsed : non-numeric character found while parsing '{raw}' as color : {c}"
                )
            }
            Self::Over255 => write!(f, "color couldn't be parsed : value over 255 in rgba"),
        }
    }
}

impl Error for ColorParsingError {}
