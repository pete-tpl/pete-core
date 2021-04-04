use crate::error::template_error::TemplateError;

use crate::common::variable::VariableStore;
use crate::context::build_context::BuildContext;
use crate::context::render_context::RenderContext;
use crate::nodes::Node;
use crate::nodes::NodeCreator;
use crate::nodes::container::ContainerNode;
use crate::nodes::comment::CommentNode;
use crate::nodes::expression::ExpressionNode;
use crate::nodes::static_node::StaticNode;
use crate::nodes::tags;

const NODE_CREATORS: [NodeCreator; 4] = [
    CommentNode::try_create_from_template,
    ExpressionNode::try_create_from_template,
    StaticNode::try_create_from_template,
    tags::try_create_from_template,
];

pub struct Engine {}

pub struct NodeBuildData {
    // end position of node. Relative to start of node.
    // does NOT consider the current offset from context
    pub end_offset: usize,
    pub is_nesting_started: bool,
    // if true, the next node will have a no linebreak in beginning
    pub is_nolinebreak_next_node: bool,
}

impl NodeBuildData {
    pub fn new(end_offset: usize, is_nesting_started: bool, is_nolinebreak_next_node: bool) -> NodeBuildData {
        NodeBuildData {
            end_offset: end_offset,
            is_nesting_started: is_nesting_started,
            is_nolinebreak_next_node: is_nolinebreak_next_node,
        }
    }
}

pub type NodeBuildResult = Result<NodeBuildData, TemplateError>;
pub type RenderResult = Result<String, TemplateError>;

impl Engine {
    pub fn new() -> Engine {
        Engine {}
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

    fn build_continuation(&self, build_context: &mut BuildContext,
                          nodes_stack: &mut Vec<Box<dyn Node>>, mut parent_node: Box<dyn Node>)
                          -> Result<Box<dyn Node>, TemplateError> {
        let data = parent_node.build(&build_context)?;
        let node = if data.is_nesting_started {
            parent_node
        } else {
            match nodes_stack.pop() {
                Some(mut upper_parent_node) => {
                    upper_parent_node.add_child(parent_node);
                    
                    Ok(upper_parent_node)
                },
                None => Err(TemplateError::create(
                        build_context.template.clone(),
                        build_context.offset,
                        String::from("Unexpected end of node stack.")))
            }?
        };
        build_context.apply_offset(data.end_offset);
        Ok(node)
    }

    fn build_new_block(&self, build_context: &mut BuildContext,
                       nodes_stack: &mut Vec<Box<dyn Node>>, mut parent_node: Box<dyn Node>)
                       -> Result<Box<dyn Node>, TemplateError> {
        let mut parsed_node = match self.parse_node(&build_context) {
            Some(n) => Ok(n),
            None => Err(TemplateError::create(
                build_context.template.clone(),
                build_context.offset,
                String::from("Cannot recognize a node"))),
        }?;
        let data = parsed_node.build(&build_context)?;
        let node = if data.is_nesting_started {
            nodes_stack.push(parent_node);
            build_context.apply_offset(data.end_offset);
            parsed_node
        } else {
            parent_node.add_child(parsed_node);
            build_context.apply_offset(data.end_offset);
            parent_node
        };
        Ok(node)
    }

    fn build(&self, template: &String) -> Result<Box<dyn Node>, TemplateError> {
        let mut nodes_stack: Vec<Box<dyn Node>> = Vec::new();
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

            let build_result = if parent_node.is_continuation(&build_context) {
                self.build_continuation(&mut build_context, &mut nodes_stack, parent_node)?
            } else {
                self.build_new_block(&mut build_context, &mut nodes_stack, parent_node)?
            };

            parent_node = build_result;
            build_context.offset += 1;
        }
        parent_node.update_end_offset();
        Ok(parent_node)
    } 

    pub fn render(&self, template: String, parameters: VariableStore) -> RenderResult {
        let parent_node = self.build(&template)?;
        let mut render_context = RenderContext::new();
        render_context.filename = String::from("(root)");
        render_context.template = template;
        render_context.parameters = parameters;
        parent_node.render(&mut render_context)
    }

    pub fn debug_print_structure(&self, template: String) -> RenderResult {
        let parent_node = self.build(&template)?;
        RenderResult::Ok(parent_node.debug_print_structure(0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_render_static_only() {
        let engine = Engine::new();
        let result = engine.render(String::from("Hello, World!"), VariableStore::new());
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
            VariableStore::new());
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
            VariableStore::new());
        match result {
            Err(e) => {
                assert_eq!(e.message, "Cannot recognize a node");
            },
            Ok(_) => {
                panic!("Rendering must have failed.");
            }
        }
    }

    #[test]
    fn test_engine_render_no_line_breaks_no_start_no_end() {
        let engine = Engine::new();
        let result = engine.render(
            String::from("Hello, World!\n{# comment #}\nNice to meet you"),
            VariableStore::new());
        match result {
            Err(e) => { panic!("Failed to render a template: {}", e) },
            Ok(result) => {
                assert_eq!(result, "Hello, World!\n\nNice to meet you");
            }
        }
    }

    #[test]
    fn test_engine_render_no_line_breaks_start_no_end() {
        let engine = Engine::new();
        let result = engine.render(
            String::from("Hello, World!\n{#- comment #}\nNice to meet you"),
            VariableStore::new());
        match result {
            Err(e) => { panic!("Failed to render a template: {}", e) },
            Ok(result) => {
                assert_eq!(result, "Hello, World!\nNice to meet you");
            }
        }
    }

    #[test]
    fn test_engine_render_no_line_breaks_no_start_end() {
        let engine = Engine::new();
        let result = engine.render(
            String::from("Hello, World!\n{# comment -#}\nNice to meet you"),
            VariableStore::new());
        match result {
            Err(e) => { panic!("Failed to render a template: {}", e) },
            Ok(result) => {
                assert_eq!(result, "Hello, World!\nNice to meet you");
            }
        }
    }

    #[test]
    fn test_engine_render_no_line_breaks_start_end() {
        let engine = Engine::new();
        let result = engine.render(
            String::from("Hello, World!\n{#- comment -#}\nNice to meet you"),
            VariableStore::new());
        match result {
            Err(e) => { panic!("Failed to render a template: {}", e) },
            Ok(result) => {
                assert_eq!(result, "Hello, World!Nice to meet you");
            }
        }
    }

    #[test]
    fn test_engine_render_no_line_breaks_only_once() {
        let engine = Engine::new();
        let result = engine.render(
            String::from("Hello, World!\n\n{#- comment -#}\n\nNice to meet you"),
            VariableStore::new());
        match result {
            Err(e) => { panic!("Failed to render a template: {}", e) },
            Ok(result) => {
                assert_eq!(result, "Hello, World!\n\nNice to meet you");
            }
        }
    }

    #[test]
    fn test_engine_render_continuation_blocks() {
        let engine = Engine::new();
        let result = engine.render(
            String::from("Hello, {% if 4 + 2 %}test{% endif %} 123"),
            VariableStore::new());
        match result {
            Err(e) => { panic!("Failed to render a template: {}", e) },
            Ok(result) => {
                assert_eq!(result, "Hello, test 123");
            }
        }
    }
}