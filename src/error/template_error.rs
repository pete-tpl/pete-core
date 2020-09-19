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
        let mut template_remain = self.template.clone();
        let mut abs_offset = 0;
        let mut line_nr = 1;
        loop {
            let pos = match template_remain.find("\n") {
                None => template_remain.len(),
                Some(l) => l
            };
            if pos >= self.offset { // Reached the line with error
                break;
            } else if 0 == pos { // Didn't found the next position
                panic!("An infinite loop detected. {}", template_remain);
            } else if pos == template_remain.len() { // end of template
                break;
            }
            template_remain = template_remain[pos+1..].to_string();
            line_nr += 1;
            abs_offset += pos + if template_remain.len() > 0 { 1 } else {0}; // consider the linebreak char
        }
        let line_offset = self.offset - abs_offset;
        write!(f,"An error ocurred at line {}, position {}: {}", line_nr, line_offset, self.message.clone())
    }
}


impl From<std::io::Error> for TemplateError {
    fn from(err: std::io::Error) -> TemplateError {
        TemplateError::create(err.to_string(), String::new(), 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_template_error() {
        let error = TemplateError::create(String::from("Unknown tag"),
            String::from("hello,\nworld!\nhere is {%test%} a tag"),
            22);
        let message = format!("{}", error);
        assert_eq!(message, "An error ocurred at line 3, position 8: Unknown tag");

        let error = TemplateError::create(String::from("Comment is not closed"),
            String::from("hello, world {#commen"),
            13);
        let message = format!("{}", error);
        assert_eq!(message, "An error ocurred at line 1, position 13: Comment is not closed");
    }
}