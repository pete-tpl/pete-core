use crate::expressions::errors::parsing_error::ParsingError;
use crate::expressions::nodes::Node;
use crate::expressions::nodes::literal::Literal;

pub mod errors;
pub mod functions;
pub mod nodes;

pub fn parse(_string: String) -> Result<Box<dyn Node>, ParsingError> {
    Ok(Box::new(Literal::new_from_str("")))
}