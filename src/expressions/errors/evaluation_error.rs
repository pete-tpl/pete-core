use std::fmt;
use std::error::Error as Error;

pub struct EvaluationError {
    pub message: String,
}

impl EvaluationError {
    pub fn new(message: String) -> EvaluationError {
        EvaluationError {
            message: message,
        }
    }

}

impl fmt::Debug for EvaluationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"An error ocurred: {}", self.message.clone())
    }
}

impl Error for EvaluationError {
    fn description(&self) -> &str {
        self.message.as_str()
    }
}

impl fmt::Display for EvaluationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"An error ocurred: {}", self.message.clone())
    }
}