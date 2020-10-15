use crate::context::render_context::RenderContext;
use crate::expressions::nodes::{Node};
use crate::expressions::errors::evaluation_error::EvaluationError;
use crate::parameter::Parameter;

//// A literal (string, number, etc) which needs no further evaluation
pub struct Literal {
    value: Parameter,
}

impl Literal {
    pub fn new(value: Parameter) -> Literal {
        Literal {
            value: value,
        }
    }

    pub fn new_from_str(string: &str) -> Literal {
        Literal::new(Parameter::new_from_str(string))
    }

    pub fn new_from_int(value: i128) -> Literal {
        Literal::new(Parameter::new_from_int(value))
    }
}

impl Node for Literal {
    fn evaluate(&self, _context: &RenderContext) -> Result<Parameter, EvaluationError> {
        Ok(self.value.clone())
    }
}
