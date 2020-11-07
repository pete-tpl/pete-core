use crate::expressions::errors::parsing_error::ParsingError;
use crate::expressions::nodes::{Node, NodeCreateResult};
use crate::expressions::nodes::NODE_CREATORS;

pub mod errors;
pub mod functions;
pub mod nodes;

fn get_parsed_node(string_remain: String, offset: usize) -> Result<(Box<dyn Node>, usize), ParsingError> {
    for node_creator in &NODE_CREATORS {
        match node_creator(string_remain.clone(), offset) {
            NodeCreateResult::Some(r) => {
                return Ok(r);
            },
            NodeCreateResult::Err(e) => {
                return Err(ParsingError::new(offset, format!("An error occurred on parsing the expresion: {}", e)));
            },
            NodeCreateResult::None => {}, // proceed with iteration over node creators
        }
    }
    Err(ParsingError::new(offset, format!("Cannot parse the part of expression: \"{}\"", string_remain)))
}

pub fn parse(string: String) -> Result<Box<dyn Node>, ParsingError> {
    let mut string_remain = string.clone();
    let mut offset: usize = 0;
    let mut prev_string_remain_len = string_remain.len() + 1;
    let mut nodes_stack: Vec<Box<dyn Node>> = Vec::new();
    while string_remain.len() > 0 {
        if string_remain.len() >= prev_string_remain_len {
            return Err(ParsingError::new(offset, String::from("An infinite loop detected")));
        }
        prev_string_remain_len = string_remain.len();
        if string_remain.starts_with(" ") {
            string_remain = string_remain[1..].to_string();
            offset += 1;
            continue;
        }

        let node = match get_parsed_node(string_remain.clone(), offset) {
            Ok(r) => {
                let (parsed_node, offset_increment) = r;
                offset += offset_increment + 1;
                string_remain = string_remain[offset_increment+1..].to_string();
                Ok(parsed_node)
            },
            Err(e) => Err(e),
        }?;

        let last_node = if !node.is_operator() && nodes_stack.len() >= 2 {
            let mut operator = nodes_stack.pop().unwrap();
            if !operator.is_operator() {
                return Err(ParsingError::new(offset, format!("Expected the previous node to be operator")));
            }
            let first_operand = nodes_stack.pop();
            operator.set_binary_operands([first_operand, Some(node)]);
            operator
        } else {
            node
        };
        nodes_stack.push(last_node);

    }
    match nodes_stack.pop() {
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

    #[test]
    fn test_expressions_parse_sum_of_int() {
        let literal = match parse(String::from("3 + 2 + 8")) {
            Ok(l) => l,
            Err(e) => panic!("Expected a literal, got an error: {}", e)
        };
        let param = match literal.evaluate(&RenderContext::new()) {
            Ok(p) => p,
            Err(e) => panic!("Expected a parameter, got an error: {}", e)
        };
        assert_eq!(param.get_int_value(), Some(13));
    }

    #[test]
    fn test_expressions_parse_logical_and() {
        let literal = match parse(String::from("0 and 0")) {
            Ok(l) => l,
            Err(e) => panic!("Expected a literal, got an error: {}", e)
        };
        let param = match literal.evaluate(&RenderContext::new()) {
            Ok(p) => p,
            Err(e) => panic!("Expected a parameter, got an error: {}", e)
        };
        assert_eq!(param.get_boolean_value(), false);
    }
}