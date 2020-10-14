use std::fmt;
use std::error::Error as Error;

pub struct ParsingError {
    pub message: String,
    pub offset: usize,
}

impl ParsingError {
    pub fn new(offset: usize, message: String) -> ParsingError {
        ParsingError {
            message: message,
            offset: offset,
        }
    }

}

impl fmt::Debug for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"An error ocurred at position {}: {}", self.offset, self.message.clone())
    }
}

impl Error for ParsingError {
    fn description(&self) -> &str {
        self.message.as_str()
    }
}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"An error ocurred at position {}: {}", self.offset, self.message.clone())
    }
}