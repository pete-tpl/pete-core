use crate::context::render_context::RenderContext;
use crate::expressions::nodes::{BinaryOperands, Node, NodeCreateResult};
use crate::expressions::errors::evaluation_error::EvaluationError;
use crate::expressions::errors::parsing_error::ParsingError;
use crate::common::variable::Variable;

//// A literal (string, number, etc) which needs no further evaluation
pub struct Literal {
    value: Variable,
}

impl Literal {
    pub fn new(value: Variable) -> Literal {
        Literal {
            value: value,
        }
    }

    pub fn new_from_bool(value: bool) -> Literal {
        Literal::new(Variable::new_from_boolean(value))
    }

    pub fn new_from_str(string: &str) -> Literal {
        Literal::new(Variable::new_from_str(string))
    }

    pub fn new_from_int(value: i128) -> Literal {
        Literal::new(Variable::new_from_int(value))
    }

    pub fn new_from_float(value: f64) -> Literal {
        Literal::new(Variable::new_from_float(value))
    }
}

pub fn try_create_from_string(expression: String, offset: usize) -> NodeCreateResult {
    let c = expression.chars().nth(0);
    if c.is_none() {
        return NodeCreateResult::None;
    }
    let c = c.unwrap();
    if c.is_digit(10) {
        return try_create_numeric_literal(expression.clone(), offset);
    } else if c == '"' {
        return try_create_string_literal(expression.clone(), offset);
    }
    NodeCreateResult::None
}

fn try_create_numeric_literal(expression: String, offset: usize) -> NodeCreateResult {
    let mut last_digit_index: usize = 0;
    loop {
        match expression.chars().nth(last_digit_index+1) {
            Some(char_at_index) => {
                if char_at_index.is_digit(10) {
                    last_digit_index += 1;
                } else {
                    break;
                }
            },
            None => { break; }
        };
    }
    let number = expression[0..last_digit_index+1].to_string();
    match number.parse::<i32>() {
        Ok(n) => NodeCreateResult::Some((Box::new(Literal::new_from_int(n.into())), last_digit_index)),
        Err(e) => NodeCreateResult::Err(ParsingError::new(offset, format!("Cannot convert char \"{}\" to integer: {}", offset, e))),
    }
}

fn try_create_string_literal(expression: String, offset: usize) -> NodeCreateResult {
    let exp = expression[1..].to_string();
    match exp.find("\"") {
        Some(pos) => {
            let string = &exp[..pos];
            NodeCreateResult::Some((Box::new(Literal::new_from_str(&string)), pos+1))
        },
        None => NodeCreateResult::Err(ParsingError::new(offset, format!("String is not closed"))),
    }
}

impl Node for Literal {
    fn evaluate(&self, _context: &RenderContext) -> Result<Variable, EvaluationError> {
        Ok(self.value.clone())
    }

    fn is_operator(&self) -> bool {
        false
    }

    fn set_binary_operands(&mut self, _operands: BinaryOperands) {

    }
}
