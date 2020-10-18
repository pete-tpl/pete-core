use crate::expressions::errors::parsing_error::ParsingError;
use crate::expressions::nodes::{Node, NodeCreator, NodeCreateResult};
use crate::expressions::nodes::literal;
use crate::expressions::nodes::variable;

pub mod errors;
pub mod functions;
pub mod nodes;

const NODE_CREATORS: [NodeCreator; 2] = [
    literal::try_create_from_string,
    variable::try_create_from_string,
];

pub fn parse(string: String) -> Result<Box<dyn Node>, ParsingError> {
    let mut string_remain = string.clone();
    let mut offset: usize = 0;
    let mut result: Option<Box<dyn Node>> = None;
    while string_remain.len() > 0 {
        let starts_with_space = match string_remain.chars().nth(0) {
            None => false,
            Some(c) => c == ' ',
        };
        if starts_with_space {
            string_remain = string_remain[1..].to_string();
            offset += 1;
        }
        for node_creator in &NODE_CREATORS {
            match node_creator(string_remain.clone(), offset) {
                NodeCreateResult::Some(r) => {
                    let (parsed_node, offset_increment) = r;
                    result = Some(parsed_node);
                    offset += offset_increment + 1;
                    string_remain = string_remain[offset_increment+1..].to_string();
                },
                NodeCreateResult::Err(e) => {
                    return Err(ParsingError::new(offset, format!("An error occurred on parsing the expresion: {}", e)));
                },
                NodeCreateResult::None => {}, // proceed with iteration over node creators
            }
        }
        if result.is_none() {
            return Err(ParsingError::new(offset, format!("Cannot parse the part of expression: \"{}\"", string_remain)));
        }
    }
    match result {
        Some(r) => Ok(r),
        None => Err(ParsingError::new(0, format!("Failed to parse an expression: \"{}\"", string)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::render_context::RenderContext;

    #[test]
    fn test_expressions_parse_string_literal_only() {
        let literal = match parse(String::from(" \"hello, world!\"  ")) {
            Ok(l) => l,
            Err(e) => panic!("Expected a literal, got an error: {}", e)
        };
        let param = match literal.evaluate(&RenderContext::new()) {
            Ok(p) => p,
            Err(e) => panic!("Expected a parameter, got an error: {}", e)
        };
        assert_eq!(param.as_string(), "hello, world!");
    }

    #[test]
    fn test_expressions_parse_int_literal_only() {
        let literal = match parse(String::from(" 123  ")) {
            Ok(l) => l,
            Err(e) => panic!("Expected a literal, got an error: {}", e)
        };
        let param = match literal.evaluate(&RenderContext::new()) {
            Ok(p) => p,
            Err(e) => panic!("Expected a parameter, got an error: {}", e)
        };
        assert_eq!(param.get_int_value(), Some(123));
    }
}