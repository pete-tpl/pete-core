use crate::context::render_context::RenderContext;
use crate::expressions::errors::evaluation_error::EvaluationError;
use crate::expressions::nodes::{BinaryOperands, Node, NodeCreateResult};
use crate::common::variable::{Variable as CommonVariable};

//// A variable from context
pub struct Variable {
    variable_name: String,
}

pub fn try_create_from_string(expression: String, _offset: usize) -> NodeCreateResult {
    let mut current_char = expression.chars().nth(0);
    let is_first_char_alphabetic = match current_char {
        None => false,
        Some(c) => c.is_alphabetic()
    };
    if !is_first_char_alphabetic {
        return NodeCreateResult::None;
    }

    let mut cursor: usize = 0;
    while cursor < expression.len() {
        current_char = expression.chars().nth(cursor+1);
        let is_valid_variable_char = match current_char {
            None => false,
            Some(c) => c.is_alphanumeric() || '_' == c  || '-' == c
        };
        if !is_valid_variable_char {
            break;
        }
        cursor += 1;
    }

    // If bracket opens after the last character - it's a function, not a variable
    let is_function_call = match expression.chars().nth(cursor+1) {
        None => false,
        Some(c) => '(' == c
    };
    if is_function_call {
        return NodeCreateResult::None;
    }

    let node = Variable::new(expression[..cursor+1].to_string());
    return NodeCreateResult::Some((Box::new(node), cursor));
}

impl Variable {
    pub fn new(variable_name: String) -> Variable {
        Variable {
            variable_name: variable_name,
        }
    }
}

impl Node for Variable {
    fn evaluate(&self, context: &RenderContext) -> Result<CommonVariable, EvaluationError> {
        match context.parameters.get(&self.variable_name) {
            Some(p) => Ok(p.clone()),
            None => Err(EvaluationError::new(format!("Variable not found: {}", self.variable_name)))
        }
    }

    fn is_operator(&self) -> bool {
        false
    }

    fn set_binary_operands(&mut self, _operands: BinaryOperands) {
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_context() -> RenderContext {
        let mut context = RenderContext::new();
        context.parameters.insert(String::from("user1"), CommonVariable::new_from_str("Alpha"));
        context.parameters.insert(String::from("user2"), CommonVariable::new_from_str("Bravo"));
        context.parameters.insert(String::from("user3"), CommonVariable::new_from_str("Charlie"));
        context
    }

    #[test]
    fn test_expressions_nodes_variable_correct() {
        let result = match try_create_from_string(String::from("user2 otherstuff"), 0) {
            NodeCreateResult::Some(result) => result,
            NodeCreateResult::Err(e) => panic!("Exprected a result, got an error: {}", e),
            NodeCreateResult::None => panic!("Exprected a result, got None"),
        };
        let (node, cursor) = result;
        assert_eq!(cursor, 4);

        let context = get_context();
        let param = match node.evaluate(&context) {
            Ok(n) => n,
            Err(e) => panic!("Expected a parameter, got an error: {}", e),
        };
        assert_eq!(param.get_string_value(), String::from("Bravo"));
    }

    #[test]
    fn test_expressions_nodes_variable_non_existing_var() {
        let result = match try_create_from_string(String::from("user4 otherstuff"), 0) {
            NodeCreateResult::Some(result) => result,
            NodeCreateResult::Err(e) => panic!("Exprected a result, got an error: {}", e),
            NodeCreateResult::None => panic!("Exprected a result, got None"),
        };
        let (node, cursor) = result;
        assert_eq!(cursor, 4);

        let context = get_context();
        let err = match node.evaluate(&context) {
            Ok(_) => panic!("Expected an error, but got a node"),
            Err(e) => e,
        };
        assert_eq!(err.message, String::from("Variable not found: user4"));
    }

    #[test]
    fn test_expressions_nodes_variable_function() {
        match try_create_from_string(String::from("my_function(arg1, arg2) abc"), 0) {
            NodeCreateResult::Some(_) => panic!("Exprected None, but got a result"),
            NodeCreateResult::Err(e) => panic!("Exprected None, but got an error: {}", e),
            NodeCreateResult::None => {},
        }
    }

    #[test]
    fn test_expressions_nodes_variable_not_var_string_literal() {
        match try_create_from_string(String::from("\"hello\" stuff"), 0) {
            NodeCreateResult::Some(_) => panic!("Exprected None, but got a result"),
            NodeCreateResult::Err(e) => panic!("Exprected None, but got an error: {}", e),
            NodeCreateResult::None => {},
        }
    }

    #[test]
    fn test_expressions_nodes_variable_not_var_int_literal() {
        match try_create_from_string(String::from("1234 stuff"), 0) {
            NodeCreateResult::Some(_) => panic!("Exprected None, but got a result"),
            NodeCreateResult::Err(e) => panic!("Exprected None, but got an error: {}", e),
            NodeCreateResult::None => {},
        }
    }
}