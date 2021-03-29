use crate::context::build_context::BuildContext;
use crate::context::render_context::RenderContext;
use crate::engine::{NodeBuildData, NodeBuildResult, RenderResult};
use crate::nodes::{BaseNode, Node, COMMENT_START, DYNAMIC_BLOCK_STARTS, EXPRESSION_START, TAG_START};

use derive_macro::HasBaseNode;

#[derive(HasBaseNode)]
pub struct StaticNode {
    base_node: BaseNode,
    content: String,
}

impl StaticNode {
    fn create() -> StaticNode {
        StaticNode{
            base_node: BaseNode::new(),
            content: String::new(),
        }
    }

    pub fn try_create_from_template(template: &String) -> Option<Box<dyn Node>> {
        if template.starts_with(COMMENT_START) || template.starts_with(TAG_START) || template.starts_with(EXPRESSION_START) {
            None
        } else {
            Some(Box::from(StaticNode::create()))
        }
    }
}

impl Node for StaticNode {
    fn add_child(&mut self, _child: Box<dyn Node>) {
        panic!("Cannot add a child to static node");
    }

    fn build(&mut self, context: &BuildContext) -> NodeBuildResult {
        let mut end_pos = context.template_remain.len() - 1;
        for start_token in &DYNAMIC_BLOCK_STARTS {
            match context.template_remain.find(start_token) {
                Some(p) => {
                    if p < end_pos {
                        end_pos = p - 1;
                    }
                },
                None => {},
            }
        }

        self.base_node.start_offset = context.offset;
        self.base_node.end_offset = context.offset + end_pos;
        self.content = context.template_remain[0..end_pos+1].to_string();
        Ok(NodeBuildData::new(end_pos, false, false))
    }

    fn is_static(&self) -> bool {
        true
    }

    fn is_continuation(&self, _context: &BuildContext) -> bool {
        return false;
    }

    fn is_control_node(&self) -> bool {
        return false;
    }

    fn render(&self, context: &mut RenderContext) -> RenderResult {
        let striped_content = match context.previous_has_nolinebreak_end {
            true => match self.content.strip_prefix("\n") {
                Some(s) => Some(s.to_string()),
                None => None,
            },
            false => None
        };
        let content = match striped_content {
            Some(s) => s,
            None => self.content.clone(),
        };
        Ok(content)
    }

    fn get_name(&self) -> &str {
        return "static";
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nodes_static_try_create_success() {
        let node = StaticNode::try_create_from_template(&String::from("the rest{# a comment #}"));
        assert_eq!(node.is_some(), true);
    }

    #[test]
    fn test_nodes_static_try_create_failure() {
        let node = StaticNode::try_create_from_template(&String::from("{# a comment #}\nthe rest"));
        assert_eq!(node.is_none(), true);
    }


    #[test]
    fn test_nodes_static_render_static_only() {
        let mut node = StaticNode::create();
        let mut context = BuildContext::new();
        context.template_remain = String::from("Hello, World!");
        let result = node.build(&context);
        match result {
            Ok(data) => {
                assert_eq!(data.end_offset, 12);
                assert_eq!(data.is_nesting_started, false);
            },
            _ => panic!("Failed to build a node")
        }
        match node.render(&mut RenderContext::new()) {
            Ok(string) => {
                assert_eq!(String::from("Hello, World!"), string);
            },
            Err(e) => panic!("Expected to render a node, but got an error: {}", e)
        }
    }

}