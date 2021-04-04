pub mod condition;

use crate::nodes::{Node, NodeCreator};
use crate::nodes::tags::condition::ConditionNode;

pub const TAG_START: &str = "{%";
pub const TAG_END: &str = "%}";

pub const NODE_CREATORS: [NodeCreator; 1] = [
    ConditionNode::try_create_from_template,
];


pub fn try_create_from_template(template: &String) -> Option<Box<dyn Node>> {
    if !template.starts_with(TAG_START) {
        return None;
    }
    
    for creator in &NODE_CREATORS {
        match creator(&template) {
            Some(t) => { return Some(t); },
            None => {},
        };
    }
    
    None
}