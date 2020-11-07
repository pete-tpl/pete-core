pub mod arithmetic;
pub mod general;
pub mod logic;

use crate::context::render_context::RenderContext;
use crate::expressions::errors::evaluation_error::EvaluationError;
use crate::expressions::errors::parsing_error::ParsingError;
use crate::common::variable::Variable;

type BinaryOperands = [Option<Box<dyn Node>>; 2];

pub const NODE_CREATORS: [NodeCreator; 3] = [
    // IMPORTANT: item order affects on node detector priority.
    // Keep the GENERAL module last
    logic::try_create_from_string,
    arithmetic::try_create_from_string,

    general::try_create_from_string,
];

pub trait Node {
    fn evaluate(&self, context: &RenderContext) -> Result<Variable, EvaluationError>;
    fn is_operator(&self) -> bool;
    fn set_binary_operands(&mut self, operands: BinaryOperands);
    fn get_type(&self) -> &str;
}

pub enum NodeCreateResult {
    Some((Box<dyn Node>, usize)),
    None,
    Err(ParsingError),
}

pub type NodeCreator = fn(expression: String, offset: usize) -> NodeCreateResult;