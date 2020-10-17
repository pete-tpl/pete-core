use crate::context::render_context::RenderContext;
use crate::expressions::nodes::{Node};
use crate::expressions::errors::evaluation_error::EvaluationError;
use crate::parameter::Parameter;

//// A variable from context
pub struct Variable {
    variable_name: String,
}

impl Variable {
    pub fn new(value: Parameter) -> Variable {
        Variable {
            variable_name: String::new(),
        }
    }

    pub fn new_from_str(string: &str) -> Variable {
        Variable::new(Parameter::new_from_str(string))
    }

    pub fn new_from_int(value: i128) -> Variable {
        Variable::new(Parameter::new_from_int(value))
    }
}

impl Node for Variable {
    fn evaluate(&self, context: &RenderContext) -> Result<Parameter, EvaluationError> {
        match context.parameters.get(self.variable_name) {
            Some(p) => Ok(p.clone()),
            None => Err(EvaluationError::new(format!("Parameter not found: {}", self.variable_name)))
        }
    }
}
