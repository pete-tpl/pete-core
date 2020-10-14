pub mod literal;

use crate::context::render_context::RenderContext;
use crate::expressions::errors::evaluation_error::EvaluationError;
use crate::parameter::Parameter;


pub trait Node {
    fn evaluate(&self, context: &RenderContext) -> Result<Parameter, EvaluationError>;
}