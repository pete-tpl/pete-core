pub mod division;
pub mod literal;
pub mod multiplication;
pub mod modulo;
pub mod subtraction;
pub mod sum;
pub mod variable;

use crate::context::render_context::RenderContext;
use crate::expressions::errors::evaluation_error::EvaluationError;
use crate::expressions::errors::parsing_error::ParsingError;
use crate::common::variable::Variable;

type BinaryOperands = [Option<Box<dyn Node>>; 2];

pub trait Node {
    fn evaluate(&self, context: &RenderContext) -> Result<Variable, EvaluationError>;
    fn is_operator(&self) -> bool;
    fn set_binary_operands(&mut self, operands: BinaryOperands);
}

pub enum NodeCreateResult {
    Some((Box<dyn Node>, usize)),
    None,
    Err(ParsingError),
}

pub type NodeCreator = fn(expression: String, offset: usize) -> NodeCreateResult;