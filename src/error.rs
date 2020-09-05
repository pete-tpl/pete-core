use std::fmt;
use std::error::Error as StdError;

pub struct Error {
    pub message: String
}

impl Error {
    pub fn create(message: String) -> Error {
        Error{
            message: message.clone()
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "An error occurred: {}",
            self.message
        )
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        self.message.as_str()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.message)
    }
}


impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::create(err.to_string())
    }
}