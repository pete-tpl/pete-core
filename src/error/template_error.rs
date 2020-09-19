use std::fmt;
use std::error::Error as Error;

pub struct TemplateError {
    pub message: String,
    pub offset: usize,
    pub template: String,
}

impl TemplateError {
    pub fn create(message: String, template: String, offset: usize) -> TemplateError {
        TemplateError{
            message: message.clone(),
            offset: offset,
            template: template.clone(),
        }
    }
}

impl fmt::Debug for TemplateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "An error occurred: {} at position {}",
            self.message,
            self.offset
        )
    }
}

impl Error for TemplateError {
    fn description(&self) -> &str {
        self.message.as_str()
    }
}

impl fmt::Display for TemplateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.message)
    }
}


impl From<std::io::Error> for TemplateError {
    fn from(err: std::io::Error) -> TemplateError {
        TemplateError::create(err.to_string(), String::new(), 0)
    }
}