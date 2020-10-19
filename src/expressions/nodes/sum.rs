use crate::context::render_context::RenderContext;
use crate::expressions::errors::evaluation_error::EvaluationError;
use crate::expressions::nodes::{BinaryOperands, Node, NodeCreateResult};
use crate::parameter::Parameter;

//// Arithmetic sum
pub struct Sum {
    operands: BinaryOperands,
}

impl Sum {
    fn new() -> Sum {
        Sum{
            operands: [None, None],
        }
    }
}

pub fn try_create_from_string(expression: String, _offset: usize) -> NodeCreateResult {
    let is_first_char_plus = match expression.chars().nth(0) {
        None => false,
        Some(c) => '+' == c
    };
    match is_first_char_plus {
        true => NodeCreateResult::Some((Box::new(Sum::new()), 1)),
        false => NodeCreateResult::None,
    }
}

impl Node for Sum {
    fn evaluate(&self, context: &RenderContext) -> Result<Parameter, EvaluationError> {
        let mut sum = Parameter::new_from_int(0);
        for (i, operand) in self.operands.iter().enumerate() {
            let operand_result_param = match operand {
                None => return Err(EvaluationError::new(format!("Operand with index '{}' is not defined", i))),
                Some(o) => o.evaluate(&context),
            }?;
            if sum.get_int_value().is_some() && operand_result_param.get_int_value().is_some() {
                sum.set_int_value(sum.get_int_value().unwrap() + operand_result_param.get_int_value().unwrap());
            } else {
                return Err(EvaluationError::new(format!("Sum operation is implemented only for integers")))
            }
            
        }
        return Ok(sum);
    }

    fn is_operator(&self) -> bool {
        true
    }

    fn set_binary_operands(&mut self, operands: BinaryOperands) {
        self.operands = operands;
    }
}
