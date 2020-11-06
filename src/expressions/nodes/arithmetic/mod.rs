pub mod division;
pub mod multiplication;
pub mod modulo;
pub mod subtraction;
pub mod sum;
pub mod variable;

use crate::expressions::nodes::{NodeCreator, NodeCreateResult};

const NODE_CREATORS: [NodeCreator; 6] = [
    division::try_create_from_string,
    modulo::try_create_from_string,
    multiplication::try_create_from_string,
    subtraction::try_create_from_string,
    sum::try_create_from_string,
    variable::try_create_from_string,
];

pub fn try_create_from_string(string_remain: String, offset: usize) -> NodeCreateResult {
    for node_creator in &NODE_CREATORS {
        match node_creator(string_remain.clone(), offset) {
            NodeCreateResult::Some(r) => return NodeCreateResult::Some(r),
            NodeCreateResult::Err(e) => return NodeCreateResult::Err(e),
            NodeCreateResult::None => {},
        }
    }
    NodeCreateResult::None
}
