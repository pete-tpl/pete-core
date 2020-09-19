use crate::error::template_error::TemplateError;
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

pub enum NodeBuildResult {
    EndOfNode(usize),
    NestedNode(usize),
    Error(TemplateError),
}

pub enum RenderResult {
    Ok(String),
    TemplateError(TemplateError)
}

impl Engine {
    pub fn new() -> Engine {
        Engine {

        }
    }

    pub fn render(&self, template: String, parameters: ParameterStore) -> RenderResult {
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
                return RenderResult::TemplateError(TemplateError::create("Cannot recognize a node".to_string(), template.clone(), cursor));
            }

            let mut parsed_node = parsed_node.unwrap();
            match parsed_node.build(&template_remain, cursor) {
                NodeBuildResult::EndOfNode(offset) => {
                    template_remain = template_remain[offset+1..].to_string();
                    parent_node.add_child(parsed_node);
                    cursor += offset;
                },
                NodeBuildResult::Error(err) => {
                    return RenderResult::TemplateError(err)
                },
                _ => {
                    return RenderResult::TemplateError(TemplateError::create(
                        "Rendering results except NodeBuildResult::EndOfNode are not implemented".to_string(),
                        template.clone(),
                        cursor));
                }
            }
        }
        parent_node.render(&parameters)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_render_static_only() {
        let engine = Engine::new();
        let result = engine.render(String::from("Hello, World!"), ParameterStore::new());
        match result {
            RenderResult::TemplateError(_) => { panic!("Failed to render a template") },
            RenderResult::Ok(result) => {
                assert_eq!(result, "Hello, World!");
            }
        }
    }

    #[test]
    fn test_engine_render_static_and_comment() {
        let engine = Engine::new();
        let result = engine.render(String::from("Hello, World!{# Some comment here #} Nice to meet you."), ParameterStore::new());
        match result {
            RenderResult::TemplateError(_) => { panic!("Failed to render a template") },
            RenderResult::Ok(result) => {
                assert_eq!(result, "Hello, World! Nice to meet you.");
            }
        }
    }

    #[test]
    fn test_engine_render_static_and_unknown_tag() {
        let engine = Engine::new();
        let result = engine.render(String::from("Hello, World!{% unknown %}unkn0wn{% endunknown %}Nice to meet you."), ParameterStore::new());
        match result {
            RenderResult::TemplateError(e) => {
                assert_eq!(e.message, "Cannot recognize a node");
            },
            RenderResult::Ok(_) => {
                panic!("Rendering must have failed.");
            }
        }
        
    }
}