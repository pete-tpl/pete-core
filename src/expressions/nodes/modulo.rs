use crate::context::render_context::RenderContext;
use crate::expressions::errors::evaluation_error::EvaluationError;
use crate::expressions::nodes::{BinaryOperands, Node, NodeCreateResult};
use crate::parameter::Parameter;

//// Modulo
pub struct Modulo {
    operands: BinaryOperands,
}

impl Modulo {
    fn new() -> Modulo {
        Modulo{
            operands: [None, None],
        }
    }
}

pub fn try_create_from_string(expression: String, _offset: usize) -> NodeCreateResult {
    let is_first_char_plus = match expression.chars().nth(0) {
        None => false,
        Some(c) => '%' == c
    };
    match is_first_char_plus {
        true => NodeCreateResult::Some((Box::new(Modulo::new()), 1)),
        false => NodeCreateResult::None,
    }
}

impl Node for Modulo {
    fn evaluate(&self, context: &RenderContext) -> Result<Parameter, EvaluationError> {
        for (i, operand) in self.operands.iter().enumerate() {
            match operand {
                None => Err(EvaluationError::new(format!("Operand with index '{}' is not defined", i))),
                Some(_) => Ok(()),
            }?;
        }
        let dividend = self.operands[0].as_ref().unwrap().evaluate(&context)?;
        let divisor = self.operands[1].as_ref().unwrap().evaluate(&context)?;
        if dividend.get_int_value().is_none() {
            return Err(EvaluationError::new(format!("Dividend is not an integer: {}", dividend.get_string_value())))
        } else if divisor.get_int_value().is_none() {
            return Err(EvaluationError::new(format!("Divisor is not an integer: {}", divisor.get_string_value())))
        }
        
        let dividend = dividend.get_int_value().unwrap();
        let divisor = divisor.get_int_value().unwrap();
        if dividend < 0 {
            return Err(EvaluationError::new(format!("Dividend must be greater or equal to zero, but it is: {}", dividend)))
        } else if divisor <= 0 {
            return Err(EvaluationError::new(format!("Divisor must be greater than zero, but it is: {}", divisor)))
        }

        Ok(Parameter::new_from_int(dividend % divisor))
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
    fn test_expressions_node_modulo_try_create_from_string_valid() {
        match try_create_from_string(String::from("% 2"), 0) {
            NodeCreateResult::Some(_) => {},
            NodeCreateResult::None => panic!("Expected an operator, got None"),
            NodeCreateResult::Err(e) => panic!("Expected an operator, got an error: {}", e),
        };
    }

    #[test]
    fn test_expressions_node_modulo_try_create_from_string_none() {
        match try_create_from_string(String::from("+ 2"), 0) {
            NodeCreateResult::Some(_) => panic!("Expected None, got Result"),
            NodeCreateResult::None => {},
            NodeCreateResult::Err(e) => panic!("Expected an operator, got an error: {}", e),
        };
    }

    #[test]
    fn test_expressions_node_modulo_two_ints() {
        let mut operator = Modulo::new();
        operator.set_binary_operands([
            Some(Box::from(Literal::new_from_int(23))),
            Some(Box::from(Literal::new_from_int(7))),
        ]);
        let param = match operator.evaluate(&RenderContext::new()) {
            Ok(p) => p,
            Err(e) => panic!("Expected an integer-type parameter, got an error: {}", e),
        };
        assert_eq!(param.get_int_value(), Some(2));
    }

    #[test]
    fn test_expressions_node_modulo_int_float() {
        let mut operator = Modulo::new();
        operator.set_binary_operands([
            Some(Box::from(Literal::new_from_int(23))),
            Some(Box::from(Literal::new_from_float(7.1))),
        ]);
        let err = match operator.evaluate(&RenderContext::new()) {
            Ok(_) => panic!("Expected an error, but got an operator"),
            Err(e) => e,
        };
        assert_eq!(err.message, "Divisor is not an integer: 7.1");
    }
}