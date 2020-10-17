pub mod literal;

use crate::context::render_context::RenderContext;
use crate::expressions::errors::evaluation_error::EvaluationError;
use crate::expressions::errors::parsing_error::ParsingError;
use crate::parameter::Parameter;


pub trait Node {
    fn evaluate(&self, context: &RenderContext) -> Result<Parameter, EvaluationError>;
}

pub enum NodeCreateResult {
    Some((Box<dyn Node>, usize)),
    None,
    Err(ParsingError),
}

pub type NodeCreator = fn(expression: String, offset: usize) -> NodeCreateResult;