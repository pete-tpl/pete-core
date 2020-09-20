use crate::error::template_error::TemplateError;

use crate::parameter::ParameterStore;
use crate::context::build_context::BuildContext;
use crate::context::render_context::RenderContext;
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

pub type RenderResult = Result<String, TemplateError>;

impl Engine {
    pub fn new() -> Engine {
        Engine {

        }
    }

    fn parse_node(&self, build_context: &BuildContext) -> Option<Box<dyn Node>> {
        for node_creator in NODE_CREATORS.iter() {
            let parsed_node = node_creator(&build_context.template_remain);
            if parsed_node.is_some() {
                return parsed_node;
            }
        }
        None
    }

    fn build_node(&self, build_context: &mut BuildContext, mut parsed_node: Box<dyn Node>, parent_node: &mut Box<dyn Node>) -> Result<(), TemplateError> {
        match parsed_node.build(&build_context) {
            NodeBuildResult::EndOfNode(offset) => {
                build_context.template_remain = build_context.template_remain[offset+1..].to_string();
                parent_node.add_child(parsed_node);
                build_context.offset += offset;
                Ok(())
            },
            NodeBuildResult::Error(err) => {
                return Err(err)
            },
            _ => {
                return Err(TemplateError::create(
                    build_context.template.clone(),
                    build_context.offset,
                    String::from("Rendering results except NodeBuildResult::EndOfNode are not implemented")));
            }
        }
    }

    fn build(&self, template: &String) -> Result<Box<dyn Node>, TemplateError> {
        let mut parent_node:Box<dyn Node> = Box::from(ContainerNode::create());
        let mut build_context = BuildContext::new();
        build_context.template = template.clone();
        build_context.template_remain = template.clone();
        let mut prev_template_remain_len = build_context.template_remain.len()+1;
        while build_context.template_remain.len() > 0 {
            if build_context.template_remain.len() >= prev_template_remain_len {
                panic!("An infinite loop detected.")
            }
            prev_template_remain_len = build_context.template_remain.len();

            let parsed_node = self.parse_node(&build_context);
            if parsed_node.is_none() {
                return Err(TemplateError::create(
                    template.clone(),
                    build_context.offset,
                    String::from("Cannot recognize a node")));
            }
            self.build_node(&mut build_context, parsed_node.unwrap(), &mut parent_node)?;

        }
        Ok(parent_node)
    } 

    pub fn render(&self, template: String, parameters: ParameterStore) -> RenderResult {
        let parent_node = self.build(&template)?;
        let mut render_context = RenderContext::new();
        render_context.filename = String::from("(root)");
        render_context.template = template;
        render_context.parameters = parameters;
        parent_node.render(&render_context)
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
            Err(e) => { panic!("Failed to render a template: {}", e) },
            Ok(result) => {
                assert_eq!(result, "Hello, World!");
            }
        }
    }

    #[test]
    fn test_engine_render_static_and_comment() {
        let engine = Engine::new();
        let result = engine.render(
            String::from("Hello, World!{# Some comment here #} Nice to meet you."),
            ParameterStore::new());
        match result {
            Err(e) => { panic!("Failed to render a template: {}", e) },
            Ok(result) => {
                assert_eq!(result, "Hello, World! Nice to meet you.");
            }
        }
    }

    #[test]
    fn test_engine_render_static_and_unknown_tag() {
        let engine = Engine::new();
        let result = engine.render(
            String::from("Hello, World!{% unknown %}unkn0wn{% endunknown %}Nice to meet you."),
            ParameterStore::new());
        match result {
            Err(e) => {
                assert_eq!(e.message, "Cannot recognize a node");
            },
            Ok(_) => {
                panic!("Rendering must have failed.");
            }
        }
        
    }
}