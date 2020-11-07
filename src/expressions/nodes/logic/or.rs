use crate::context::render_context::RenderContext;
use crate::expressions::errors::evaluation_error::EvaluationError;
use crate::expressions::nodes::{BinaryOperands, Node, NodeCreateResult};
use crate::common::variable::Variable;

//// Logical Or
pub struct Or {
    operands: BinaryOperands,
}

impl Or {
    fn new() -> Or {
        Or{
            operands: [None, None],
        }
    }
}

pub fn try_create_from_string(expression: String, _offset: usize) -> NodeCreateResult {
    let result = expression.starts_with("||") ||
        (expression.starts_with("or") && 
            (expression.len() <= 2 ||
                !expression.chars().nth(2).unwrap().is_alphabetic()));
    match result {
        true => NodeCreateResult::Some((Box::new(Or::new()), 1)),
        false => NodeCreateResult::None,
    }
}

impl Node for Or {
    fn evaluate(&self, context: &RenderContext) -> Result<Variable, EvaluationError> {
        for (i, operand) in self.operands.iter().enumerate() {
            match operand {
                None => Err(EvaluationError::new(format!("Operand with index '{}' is not defined", i))),
                Some(_) => Ok(()),
            }?;
        }
        let operand1 = self.operands[0].as_ref().unwrap().evaluate(&context)?;
        let operand2 = self.operands[1].as_ref().unwrap().evaluate(&context)?;
        let result = Variable::new_from_boolean(operand1.get_boolean_value() || operand2.get_boolean_value());

        return Ok(result);
    }

    fn is_operator(&self) -> bool {
        true
    }

    fn set_binary_operands(&mut self, operands: BinaryOperands) {
        self.operands = operands;
    }

    fn get_type(&self) -> &str {
        "or"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::render_context::RenderContext;
    use crate::expressions::nodes::general::literal::Literal;

    #[test]
    fn test_expressions_node_logic_or_try_create_from_string_valid() {
        match try_create_from_string(String::from("|| 2"), 0) {
            NodeCreateResult::Some(_) => {},
            NodeCreateResult::None => panic!("Expected an operator, got None"),
            NodeCreateResult::Err(e) => panic!("Expected an operator, got an error: {}", e),
        };
        match try_create_from_string(String::from("or 2"), 0) {
            NodeCreateResult::Some(_) => {},
            NodeCreateResult::None => panic!("Expected an operator, got None"),
            NodeCreateResult::Err(e) => panic!("Expected an operator, got an error: {}", e),
        };
    }

    #[test]
    fn test_expressions_node_logic_or_try_create_from_string_long_alphabetic() {
        match try_create_from_string(String::from("ornotanoperator 2"), 0) {
            NodeCreateResult::Some(_) => panic!("Expected None, got Result"),
            NodeCreateResult::None => {},
            NodeCreateResult::Err(e) => panic!("Expected an operator, got an error: {}", e),
        };
    }


    #[test]
    fn test_expressions_node_logic_or_true_false() {
        let mut operator = Or::new();
        operator.set_binary_operands([
            Some(Box::from(Literal::new_from_bool(true))),
            Some(Box::from(Literal::new_from_bool(false))),
        ]);
        let param = match operator.evaluate(&RenderContext::new()) {
            Ok(p) => p,
            Err(e) => panic!("Expected a Boolean-type parameter, got an error: {}", e),
        };
        assert_eq!(param.get_boolean_value(), true);
    }

    #[test]
    fn test_expressions_node_logic_or_true_true() {
        let mut operator = Or::new();
        operator.set_binary_operands([
            Some(Box::from(Literal::new_from_bool(true))),
            Some(Box::from(Literal::new_from_bool(true))),
        ]);
        let param = match operator.evaluate(&RenderContext::new()) {
            Ok(p) => p,
            Err(e) => panic!("Expected a Boolean-type parameter, got an error: {}", e),
        };
        assert_eq!(param.get_boolean_value(), true);
    }

    #[test]
    fn test_expressions_node_logic_or_false_false() {
        let mut operator = Or::new();
        operator.set_binary_operands([
            Some(Box::from(Literal::new_from_bool(false))),
            Some(Box::from(Literal::new_from_bool(false))),
        ]);
        let param = match operator.evaluate(&RenderContext::new()) {
            Ok(p) => p,
            Err(e) => panic!("Expected a Boolean-type parameter, got an error: {}", e),
        };
        assert_eq!(param.get_boolean_value(), false);
    }
}