use std::fmt;
use std::error::Error as Error;

pub struct TemplateError {
    pub message: String,
    pub offset: usize,
    pub template: String,
}

impl TemplateError {
    pub fn create(template: String, offset: usize, message: String) -> TemplateError {
        TemplateError{
            message: message,
            offset: offset,
            template: template,
        }
    }

    fn get_line_number_and_offset(&self) -> (usize, usize) {
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
        (line_nr, line_offset)
    }
}

impl fmt::Debug for TemplateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (line_nr, line_offset) = self.get_line_number_and_offset();
        write!(f,"An error ocurred at line {}, position {}: {}", line_nr, line_offset, self.message.clone())
    }
}

impl Error for TemplateError {
    fn description(&self) -> &str {
        self.message.as_str()
    }
}

impl fmt::Display for TemplateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (line_nr, line_offset) = self.get_line_number_and_offset();
        write!(f,"An error ocurred at line {}, position {}: {}", line_nr, line_offset, self.message.clone())
    }
}


impl From<std::io::Error> for TemplateError {
    fn from(err: std::io::Error) -> TemplateError {
        TemplateError::create(String::new(), 0, err.to_string()) // Is it needed?
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_template_error() {
        let error = TemplateError::create(
            String::from("hello,\nworld!\nhere is {%test%} a tag"),
            22,
            String::from("Unknown tag"));
        let message = format!("{}", error);
        assert_eq!(message, "An error ocurred at line 3, position 8: Unknown tag");

        let error = TemplateError::create(
            String::from("hello, world {#commen"),
            13,
            String::from("Comment is not closed"));
        let message = format!("{}", error);
        assert_eq!(message, "An error ocurred at line 1, position 13: Comment is not closed");
    }
}