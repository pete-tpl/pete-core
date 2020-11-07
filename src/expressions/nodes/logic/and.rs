use crate::context::render_context::RenderContext;
use crate::expressions::errors::evaluation_error::EvaluationError;
use crate::expressions::nodes::{BinaryOperands, Node, NodeCreateResult};
use crate::common::variable::Variable;

const WORD_FORM: &str = "and";
const SYMBOL_FORM: &str = "&&";

//// Logical And
pub struct And {
    operands: BinaryOperands,
}

impl And {
    fn new() -> And {
        And{
            operands: [None, None],
        }
    }
}

pub fn try_create_from_string(expression: String, _offset: usize) -> NodeCreateResult {
    let is_symbol_form = expression.starts_with(SYMBOL_FORM);
    let is_word_form = expression.starts_with(WORD_FORM) &&
        (expression.len() <= WORD_FORM.len() ||
            !expression.chars().nth(WORD_FORM.len()).unwrap().is_alphabetic());

    match is_symbol_form || is_word_form {
        true => {
            let keyword = if is_symbol_form { SYMBOL_FORM } else { WORD_FORM };
            NodeCreateResult::Some((Box::new(And::new()), keyword.len()))
        },
        false => NodeCreateResult::None,
    }
}

impl Node for And {
    fn evaluate(&self, context: &RenderContext) -> Result<Variable, EvaluationError> {
        for (i, operand) in self.operands.iter().enumerate() {
            match operand {
                None => Err(EvaluationError::new(format!("Operand with index '{}' is not defined", i))),
                Some(_) => Ok(()),
            }?;
        }
        let operand1 = self.operands[0].as_ref().unwrap().evaluate(&context)?;
        let operand2 = self.operands[1].as_ref().unwrap().evaluate(&context)?;
        let result = Variable::new_from_boolean(operand1.get_boolean_value() && operand2.get_boolean_value());

        return Ok(result);
    }

    fn is_operator(&self) -> bool {
        true
    }

    fn set_binary_operands(&mut self, operands: BinaryOperands) {
        self.operands = operands;
    }

    fn get_type(&self) -> &str {
        "and"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::render_context::RenderContext;
    use crate::expressions::nodes::general::literal::Literal;

    #[test]
    fn test_expressions_node_logic_and_try_create_from_string_valid() {
        match try_create_from_string(String::from("&& 2"), 0) {
            NodeCreateResult::Some(_) => {},
            NodeCreateResult::None => panic!("Expected an operator, got None"),
            NodeCreateResult::Err(e) => panic!("Expected an operator, got an error: {}", e),
        };
        match try_create_from_string(String::from("and 2"), 0) {
            NodeCreateResult::Some(_) => {},
            NodeCreateResult::None => panic!("Expected an operator, got None"),
            NodeCreateResult::Err(e) => panic!("Expected an operator, got an error: {}", e),
        };
    }

    #[test]
    fn test_expressions_node_logic_and_try_create_from_string_long_alphabetic() {
        match try_create_from_string(String::from("andnotanoperator 2"), 0) {
            NodeCreateResult::Some(_) => panic!("Expected None, got Result"),
            NodeCreateResult::None => {},
            NodeCreateResult::Err(e) => panic!("Expected an operator, got an error: {}", e),
        };
    }


    #[test]
    fn test_expressions_node_logic_and_true_false() {
        let mut operator = And::new();
        operator.set_binary_operands([
            Some(Box::from(Literal::new_from_bool(true))),
            Some(Box::from(Literal::new_from_bool(false))),
        ]);
        let param = match operator.evaluate(&RenderContext::new()) {
            Ok(p) => p,
            Err(e) => panic!("Expected a Boolean-type parameter, got an error: {}", e),
        };
        assert_eq!(param.get_boolean_value(), false);
    }

    #[test]
    fn test_expressions_node_logic_and_true_true() {
        let mut operator = And::new();
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
    fn test_expressions_node_logic_and_false_false() {
        let mut operator = And::new();
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