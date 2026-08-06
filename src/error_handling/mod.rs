//! usefull error types that represent error that are in relation with the parsing of text files

use std::{fmt::{Debug, Display}, path::Path};

/// This represent an error that happens while either opening or parsing a file.
/// The error contains the file name, the error message and, optionally, the line number where the
/// parsing error happened.
///
/// This can be created from a [`ParsingError`] using from()
#[derive(Clone)]
pub struct FileReadingError<'p> {
    filepath: &'p Path,
    message: String,
    line: Option<usize>,
}

impl <'p> FileReadingError<'p> {
    /// Create a new [`FileReadingError`] from a file path and an error message
    pub fn new(filepath: &'p Path, message: String) -> Self {
        FileReadingError {
            filepath,
            message,
            line: None,
        }
    }

    pub fn from_parsing_error(filepath: &'p Path, e:ParsingError) -> Self{
        FileReadingError {
            filepath,
            message: e.message().clone(),
            line: Some(e.line_number())
        }
    }

    /// Returns an optional containing the line number where the parsing error happened
    pub fn line_number(&self) -> Option<usize> {
        self.line
    }

    pub fn filename(&self)->&'p Path {
        self.filepath
    }

    pub fn message(&self) -> &String {
        &self.message
    }
}

impl Debug for FileReadingError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(line_number) = self.line {
            write!(
                f,
                "File \"{}\" could not be parsed at line '{line_number}' because : {}",
                self.filepath.to_str().unwrap_or("INVALID UTF-8"), self.message
            )
        } else {
            write!(
                f,
                "File \"{}\" could not be because : {}",
                self.filepath.to_str().unwrap_or("INVALID UTF-8"), self.message
            )
        }
    }
}
impl Display for FileReadingError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "File \"{}\" could not be parsed because : {}",
            self.filepath.to_str().unwrap_or("INVALID UTF-8"), self.message
        )
    }
}


/// Represent a parsing error that happened at a certain line number inside of a file
#[derive(Debug, Clone)]
pub struct ParsingError {
    message: String,
    line_number: usize,
}

impl ParsingError{
    /// Create a [`ParsingError`] from an error message and a line number
    pub fn new( message: String, line_number: usize) -> Self {
        ParsingError {
            message,
            line_number,
        }
    }
    /// Return the line number where the parsing error happened
    pub fn line_number(&self) -> usize {
        self.line_number
    }
    /// Return the error message of the parsing error
    pub fn message(&self) -> &String {
        &self.message
    }
}

impl Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "could not be parse line '{}' because : {}",
            self.line_number, self.message
        )
    }
}
