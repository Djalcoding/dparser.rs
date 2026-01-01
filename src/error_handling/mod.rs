//! usefull error types that represent error that are in relation with the parsing of text files

use std::fmt::{Debug, Display};

/// This represent an error that happens while either opening or parsing a file.
/// The error contains the file name, the error message and, optionally, the line number where the
/// parsing error happened.
///
/// This can be created from a [`ParsingError`] using from()
#[derive(Clone)]
pub struct FileReadingError {
    file: String,
    message: String,
    line: Option<usize>,
}

impl FileReadingError {
    /// Create a new [`FileReadingError`] from a file path and an error message
    pub fn new(file: String, message: String) -> Self {
        FileReadingError {
            file,
            message,
            line: None,
        }
    }

    /// Returns an optional containing the line number where the parsing error happened
    pub fn line_number(&self) -> Option<usize> {
        self.line
    }

    fn from_parsing_error(error: &ParsingError) -> Self {
        FileReadingError {
            file: error.file(),
            message: error.message(),
            line: Some(error.line_number()),
        }
    }
}

impl Debug for FileReadingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(line_number) = self.line {
            write!(
                f,
                "File \"{}\" could not be parsed at line '{line_number}' because :\n{}",
                self.file, self.message
            )
        } else {
            write!(
                f,
                "File \"{}\" could not be because :\n{}",
                self.file, self.message
            )
        }
    }
}
impl Display for FileReadingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "File \"{}\" could not be parsed  because :\n{}",
            self.file, self.message
        )
    }
}

impl From<ParsingError> for FileReadingError {
    fn from(value: ParsingError) -> Self {
        FileReadingError::from_parsing_error(&value)
    }
}

/// Represent a parsing error that happened at a certain line number inside of a file
#[derive(Debug, Clone)]
pub struct ParsingError {
    file: String,
    message: String,
    line_number: usize,
}

impl ParsingError {
    /// Create a [`ParsingError`] from a file path, an error message and a line number
    pub fn new(file: String, message: String, line_number: usize) -> Self {
        ParsingError {
            file,
            message,
            line_number,
        }
    }
    /// Return the line number where the parsing error happened
    pub fn line_number(&self) -> usize {
        self.line_number
    }
    /// Return the error message of the parsing error
    pub fn message(&self) -> String {
        self.message.clone()
    }
    /// Return the file where the parsing error happened
    pub fn file(&self) -> String {
        self.file.clone()
    }
}

impl Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "could not be parse line '{}' because :\n{}",
            self.line_number, self.message
        )
    }
}
