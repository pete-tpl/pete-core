use crate::expressions::errors::parsing_error::ParsingError;
use crate::expressions::nodes::Node;
use crate::expressions::nodes::literal::Literal;

pub mod errors;
pub mod functions;
pub mod nodes;

enum ParseMode {
    Undefined,
    StringLiteral,
    NumericLiteral,
    
}

pub fn parse(string: String) -> Result<Box<dyn Node>, ParsingError> {
    let result = string.trim();
    let mut mode: ParseMode = ParseMode::Undefined;
    let mut cursor: usize = 0;
    let mut pos_start: usize = 0;
    let string_len = result.len();
    let mut parent_node: Box<dyn Node> = Box::new(Literal::new_from_str(""));
    while cursor <= string_len {
        let c = result.chars().nth(cursor);
        if c.is_none() {
            break;
        }
        let c = c.unwrap();
        println!("TEST: {}", c);
        if c.is_digit(10) {
            match mode {
                ParseMode::Undefined => {
                    mode = ParseMode::NumericLiteral;
                    pos_start = cursor;
                },
                ParseMode::NumericLiteral => {},
                _ => {},
            }
        } else if c == '"' {
            match mode {
                ParseMode::Undefined => {
                    mode = ParseMode::StringLiteral;
                    pos_start = cursor + 1;
                },
                ParseMode::StringLiteral => {
                    mode = ParseMode::Undefined;
                    parent_node = Box::new(Literal::new_from_str(result[pos_start..cursor].as_ref()))
                }
                _ => {}
            }
        } else {
            match mode {
                ParseMode::NumericLiteral => {
                    let number = result[pos_start..cursor].to_string();
                    parent_node = convert_to_literal(number, cursor)?;
                },
                ParseMode::StringLiteral => {},
                _ => {
                    return Err(ParsingError::new(cursor, format!("Unexpected char: {}", c)));
                }
            }
        }
        cursor += 1;
    }
    match mode {
        ParseMode::NumericLiteral => {
            let number = result[pos_start..cursor].to_string();
            parent_node = convert_to_literal(number, cursor)?;
        },
        ParseMode::StringLiteral => {
            return Err(ParsingError::new(pos_start, format!("String is not closed")))
        },
        _ => {}
    }


    Ok(parent_node)
}

fn convert_to_literal(string: String, cursor: usize) -> Result<Box<Literal>, ParsingError> {
    match string.parse::<i32>() {
        Ok(n) => Ok(Box::new(Literal::new_from_int(n.into()))),
        Err(e) => Err(ParsingError::new(cursor, format!("Cannot convert char \"{}\" to integer: {}", cursor, e))),
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