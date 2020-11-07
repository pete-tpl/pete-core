pub mod and;
pub mod or;

use crate::expressions::nodes::{NodeCreator, NodeCreateResult};

const NODE_CREATORS: [NodeCreator; 2] = [
    and::try_create_from_string,
    or::try_create_from_string,
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