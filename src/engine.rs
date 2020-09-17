use crate::error::Error;
use crate::parameter::ParameterStore;

use crate::nodes::Node;
use crate::nodes::NodeCreator;
use crate::nodes::container::ContainerNode;
use crate::nodes::comment::CommentNode;
use crate::nodes::static_node::StaticNode;

const NODE_CREATORS: [NodeCreator; 2] = [
    CommentNode::try_create_from_template,
    StaticNode::try_create_from_template,
];

pub struct Engine {}

pub enum RenderResult {
    EndOfNode(usize),
    NestedNode(usize),
    Error(Error),
}

impl Engine {
    pub fn new() -> Engine {
        Engine {

        }
    }

    pub fn render(&self, template: String, parameters: ParameterStore) -> Result<String, Error> {
        let mut cursor = 0;
        let mut parent_node:Box<dyn Node> = Box::from(ContainerNode::create());
        let mut template_remain = template.clone();
        let mut prev_template_remain_len = template_remain.len()+1;
        while template_remain.len() > 0 {
            if template_remain.len() >= prev_template_remain_len {
                panic!("An infinite loop detected.")
            }
            prev_template_remain_len = template_remain.len();
            let mut parsed_node: Option<Box<dyn Node>> = None;
            for node_creator in NODE_CREATORS.iter() {
                parsed_node = node_creator(&template_remain);
                if parsed_node.is_some() {
                    break;
                }
            }

            if parsed_node.is_none() {
                return Err(Error::create("Cannot recognize a node".to_string(), Some(cursor)));
            }

            let mut parsed_node = parsed_node.unwrap();
            match parsed_node.build(&template_remain, cursor) {
                RenderResult::EndOfNode(offset) => {
                    template_remain = template_remain[offset+1..].to_string();
                    parent_node.add_child(parsed_node);
                    cursor += offset;
                },
                RenderResult::Error(err) => {
                    return Err(err)
                },
                _ => {
                    return Err(Error::create("Rendering results except RenderResult::EndOfNode are not implemented".to_string(), Some(cursor)));
                }
            }
        }
        match parent_node.render(&parameters) {
            Ok(content) => Ok(content),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_render_static_only() {
        let engine = Engine::new();
        let result = engine.render(String::from("Hello, World!"), ParameterStore::new());
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), "Hello, World!");
    }

    #[test]
    fn test_engine_render_static_and_comment() {
        let engine = Engine::new();
        let result = engine.render(String::from("Hello, World!{# Some comment here #} Nice to meet you."), ParameterStore::new());
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), "Hello, World! Nice to meet you.");
    }

    #[test]
    fn test_engine_render_static_and_unknown_tag() {
        let engine = Engine::new();
        let result = engine.render(String::from("Hello, World!{% unknown %}unkn0wn{% endunknown %}Nice to meet you."), ParameterStore::new());
        assert_eq!(result.is_err(), true);
        assert_eq!(result.unwrap_err().message, "Cannot recognize a node");
    }
}