use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct FileReadingError {
    file: String,
    message: String,
    line: Option<usize>,
}

impl FileReadingError {
    pub fn new(file: String, message: String) -> Self {
        FileReadingError {
            file,
            message,
            line: None,
        }
    }

    pub fn from_parsing_error(error: &ParsingError) -> Self {
        FileReadingError {
            file: error.file(),
            message: error.message(),
            line: Some(error.line_number()),
        }
    }

    pub fn line_number(&self) -> Option<usize> {
        self.line
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

#[derive(Debug, Clone)]
pub struct ParsingError {
    file: String,
    message: String,
    line_number: usize,
}

impl ParsingError {
    pub fn new(file: String, message: String, line_number: usize) -> Self {
        ParsingError {
            file,
            message,
            line_number,
        }
    }
    pub fn line_number(&self) -> usize {
        self.line_number
    }
    pub fn message(&self) -> String {
        self.message.clone()
    }
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
