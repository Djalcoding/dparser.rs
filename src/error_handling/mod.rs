//! Errors that might occur while trying to parse a file

use std::{
    error::Error,
    fmt::Display,
    path::{PathBuf},
};

use crate::error_handling::ColorParsingError::InvalidArgumentCount;


/// Represents an error that might occur while parsing a file
/// Everything is owned, so a clone operation may be necessary
#[derive(Debug)]
pub enum ParsingError {
    // This is not a struct because I love flat girls
    /// A syscall error, wrapper for [`std::io::Error`].
    /// Contains filepath.
    IOError {
        filepath: PathBuf,
        error: std::io::Error,
    },
    /// Occurs when a line has no seperator.
    /// Contains error location.
    NoSeperator { filepath: PathBuf, line: usize },
    /// An error that occurs when a key is not present in a [`crate::datastructure::ParsedData`].
    NonExistantEntry(String),
    /// An error that occurs when failing to parse a number.
    /// Contains error location.
    /// See [`NumberParsingError`]
    NumberError {
        filepath: PathBuf,
        line: usize,
        error: NumberParsingError,
    },
    /// An error that occurs when failing to parse text.
    /// Contains error location.
    /// See [`TextParsingError`]
    TextError {
        filepath: PathBuf,
        line: usize,
        error: TextParsingError,
    },
    /// An error that occurs when failing to parse a boolean.
    /// Contains error location.
    /// See [`BoolParsingError`]
    BoolError {
        filepath: PathBuf,
        line: usize,
        error: BoolParsingError,
    },
    /// An error that occurs when failing to parse a color.
    /// Contains error location.
    /// See [`ColorParsingError`]
    ColorError {
        filepath: PathBuf,
        line: usize,
        error: ColorParsingError,
    },
    /// Wrapper for another Error to indicate that this error happends over an array
    ArrayError(Box<ParsingError>), // Boohou heap allocation, it only happens on errors
}

impl ParsingError {
    /// Wrap an error into an array error
    /// [See Also]
    /// [`ParsingError::ArrayError`]
    pub fn array_error(self) -> Self {
        Self::ArrayError(Box::new(self))
    }
}

impl Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
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
            Self::BoolError {
                filepath,
                line,
                error,
            } => {
                write!(
                    f,
                    "Failed to parse boolean within '{}' on line {line} : {error}",
                    filepath.display()
                )
            }
            Self::ColorError {
                filepath,
                line,
                error,
            } => {
                write!(
                    f,
                    "Failed to parse color within '{}' on line {line} : {error}",
                    filepath.display()
                )
            }
            Self::NumberError {
                filepath,
                line,
                error,
            } => {
                write!(
                    f,
                    "Failed to parse number within '{}' on line {line} : {error}",
                    filepath.display()
                )
            }
            Self::TextError {
                filepath,
                line,
                error,
            } => {
                write!(
                    f,
                    "Failed to parse text within '{}' on line {line} : {error}",
                    filepath.display()
                )
            }
        }
    }
}

impl Error for ParsingError {}

#[derive(Debug, PartialEq, Clone)]
/// An error that describes a failure in number parsing
pub enum NumberParsingError {
    /// Occurs when input string cannot fit in a [`f64`]
    TooLarge(String),
    /// Occurs when any character in the input string fails [`char::is_ascii_digit()`]
    NonNumericCharacter(String, char),
}
impl Display for NumberParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NumberParsingError::NonNumericCharacter(raw, c) => {
                write!(
                    f,
                    "Non-numeric character found while parsing '{raw}' as number : '{c}' "
                )
            }
            NumberParsingError::TooLarge(raw) => {
                write!(f, "Number '{raw}' is too large to be parsed")
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
        write!(f, "{name} is not a valid boolean identifier")
    }
}
impl Error for BoolParsingError{}

#[derive(Debug, PartialEq, Clone)]
/// An error that describes a failure in text parsing
pub enum TextParsingError {
    /// An error that occurs when a the input string doesn't have atleast 2 double quotes
    NotEnoughDoubleQuotes(String),
}
impl Display for TextParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self::NotEnoughDoubleQuotes(name) = self;
        write!(f, "{name} doesn't have enough double quotes to be parsed")
    }
}
impl Error for TextParsingError{}

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
                "{found} arguments provided for color when 3 or 4 are expected"
            ),
            Self::UnknownName(name) => write!(f, "{name} is not a recognized color"),
            Self::NonNumericCharacter(raw, c) => {
                write!(
                    f,
                    "Non-numeric character found while parsing '{raw}' as color : {c}"
                )
            }
            Self::Over255 => write!(f, "A value over 255 is inside rgba color"),
        }
    }
}

impl Error for ColorParsingError{}
