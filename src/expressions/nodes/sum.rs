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
        for (i, operand) in self.operands.iter().enumerate() {
            match operand {
                None => Err(EvaluationError::new(format!("Operand with index '{}' is not defined", i))),
                Some(_) => Ok(()),
            }?;
        }
        let operand1 = self.operands[0].as_ref().unwrap().evaluate(&context)?;
        let operand2 = self.operands[1].as_ref().unwrap().evaluate(&context)?;
        let mut result = Parameter::new_from_int(0);
        if operand1.get_int_value().is_some() && operand2.get_int_value().is_some() {
            result.set_int_value(operand1.get_int_value().unwrap() + operand2.get_int_value().unwrap());
        } else if operand1.get_float_value().is_some() && operand2.get_float_value().is_some() {
            result.set_float_value(operand1.get_float_value().unwrap() + operand2.get_float_value().unwrap());
        } else {
            return Err(EvaluationError::new(format!("Unsupported types of operands for sum operator")))
        }

        return Ok(result);
    }

    fn is_operator(&self) -> bool {
        true
    }

    fn set_binary_operands(&mut self, operands: BinaryOperands) {
        self.operands = operands;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::render_context::RenderContext;
    use crate::expressions::nodes::literal::Literal;

    #[test]
    fn test_expressions_node_sum_try_create_from_string_valid() {
        match try_create_from_string(String::from("+ 2"), 0) {
            NodeCreateResult::Some(_) => {},
            NodeCreateResult::None => panic!("Expected an operator, got None"),
            NodeCreateResult::Err(e) => panic!("Expected an operator, got an error: {}", e),
        };
    }

    #[test]
    fn test_expressions_node_sum_try_create_from_string_none() {
        match try_create_from_string(String::from("- 2"), 0) {
            NodeCreateResult::Some(_) => panic!("Expected None, got Result"),
            NodeCreateResult::None => {},
            NodeCreateResult::Err(e) => panic!("Expected an operator, got an error: {}", e),
        };
    }

    #[test]
    fn test_expressions_node_sum_two_ints() {
        let mut operator = Sum::new();
        operator.set_binary_operands([
            Some(Box::from(Literal::new_from_int(7))),
            Some(Box::from(Literal::new_from_int(42))),
        ]);
        let param = match operator.evaluate(&RenderContext::new()) {
            Ok(p) => p,
            Err(e) => panic!("Expected an integer-type parameter, got an error: {}", e),
        };
        assert_eq!(param.get_int_value(), Some(49));
    }

    #[test]
    fn test_expressions_node_sum_int_float() {
        let mut operator = Sum::new();
        operator.set_binary_operands([
            Some(Box::from(Literal::new_from_int(7))),
            Some(Box::from(Literal::new_from_float(6.5))),
        ]);
        let param = match operator.evaluate(&RenderContext::new()) {
            Ok(p) => p,
            Err(e) => panic!("Expected a float-type parameter, got an error: {}", e),
        };
        assert_eq!(param.get_float_value(), Some(13.5));
    }

    #[test]
    fn test_expressions_node_sum_string_int() {
        let mut operator = Sum::new();
        operator.set_binary_operands([
            Some(Box::from(Literal::new_from_str("Hello"))),
            Some(Box::from(Literal::new_from_float(6.5))),
        ]);
        let err = match operator.evaluate(&RenderContext::new()) {
            Ok(_) => panic!("Expected an error, but got an operator"),
            Err(e) => e,
        };
        assert_eq!(err.message, "Unsupported types of operands for sum operator");
    }
}