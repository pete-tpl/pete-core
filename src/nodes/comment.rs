use crate::context::build_context::BuildContext;
use crate::context::render_context::RenderContext;
use crate::engine::{NodeBuildData, NodeBuildResult, RenderResult};
use crate::error::template_error::TemplateError;
use crate::nodes::{BaseNode, Node, COMMENT_START, COMMENT_END};

use derive_macro::HasBaseNode;

#[derive(HasBaseNode)]
pub struct CommentNode {
    base_node: BaseNode,
}

impl CommentNode {
    fn create() -> CommentNode {
        CommentNode{
            base_node: BaseNode::new(),
        }
    }

    pub fn try_create_from_template(template: &String) -> Option<Box<dyn Node>> {
        if template.starts_with(COMMENT_START) {
            Some(Box::from(CommentNode::create()))
        } else {
            None
        }
    }
}

impl Node for CommentNode {
    fn add_child(&mut self, _child: Box<dyn Node>) {
        panic!("Cannot add a child to comment node");
    }

    fn build(&mut self, context: &BuildContext) -> NodeBuildResult {
        let end_pos = context.template_remain.find(COMMENT_END);
        match end_pos {
            None => Err(TemplateError::create(
                context.template.clone(),
                context.offset,
                String::from("Comment is not closed"))),
            Some(end_pos) => {
                let end_pos_with_tag = end_pos - 1 + COMMENT_END.len();
                self.base_node.end_offset = context.offset + end_pos_with_tag;
                self.base_node.has_nolinebreak_end = context.template_remain[end_pos-1..end_pos].to_string() == "-";
                self.base_node.start_offset = context.offset;
                Ok(NodeBuildData{
                    end_offset: end_pos_with_tag,
                    is_nesting_started: false,
                    is_nolinebreak_prev_node: context.template_remain[2..3].to_string() == "-",
                    is_nolinebreak_next_node: context.template_remain[end_pos-1..end_pos].to_string() == "-",
                })
            }
        }
    }

    fn is_continuation(&self, _context: &BuildContext) -> bool {
        return false;
    }

    fn render(&self, _context: &RenderContext) -> RenderResult {
        RenderResult::Ok(String::new())
    }

    fn debug_name(&self) -> &str {
        return "comment";
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nodes_comment_try_create_success() {
        let node = CommentNode::try_create_from_template(&String::from("{# a comment #}\nthe rest"));
        assert_eq!(node.is_some(), true);
    }

    #[test]
    fn test_nodes_comment_try_create_failure() {
        let node = CommentNode::try_create_from_template(&String::from("the rest{# a comment #}"));
        assert_eq!(node.is_none(), true);
    }

    #[test]
    fn test_nodes_static_render_with_static() {
        let mut node = CommentNode::create();
        let mut context = BuildContext::new();
        context.template_remain = String::from("{# Here is comment #}World!");
        context.offset = 7;
        let result = node.build(&context);
        match result {
            Ok(data) => {
                assert_eq!(data.end_offset, 20);
                assert_eq!(data.is_nesting_started, false);
            },
            _ => panic!("Failed to build a node")
        }
        let context = RenderContext::new();
        match node.render(&context) {
            Ok(string) => {
                assert_eq!(String::new(), string);
            },
            Err(e) => panic!("Expected to render a node, but got an error: {}", e)
        }
    }

    #[test]
    fn test_nodes_static_nolinebraks() {
        let mut node = CommentNode::create();
        let mut context = BuildContext::new();
        context.template_remain = String::from("{#- Here is comment -#}World!");
        context.offset = 21;
        match node.build(&context) {
            Ok(data) => {
                assert_eq!(data.end_offset, 22);
                assert_eq!(data.is_nesting_started, false);
                assert_eq!(node.base_node.start_offset, 21);
                assert_eq!(node.base_node.end_offset, 43);
                assert_eq!(node.base_node.has_nolinebreak_beginning, true);
                assert_eq!(node.base_node.has_nolinebreak_end, true);
            },
            _ => panic!("Failed to build a node")
        }

        let mut node = CommentNode::create();
        let mut context = BuildContext::new();
        context.template_remain = String::from("{# Here is comment -#}World!");
        context.offset = 21;
        match node.build(&context) {
            Ok(data) => {
                assert_eq!(data.end_offset, 21);
                assert_eq!(data.is_nesting_started, false);
                assert_eq!(node.base_node.start_offset, 21);
                assert_eq!(node.base_node.end_offset, 42);
                assert_eq!(node.base_node.has_nolinebreak_beginning, false);
                assert_eq!(node.base_node.has_nolinebreak_end, true);
            },
            _ => panic!("Failed to build a node")
        }

        let mut node = CommentNode::create();
        let mut context = BuildContext::new();
        context.template_remain = String::from("{#- Here is comment #}World!");
        context.offset = 21;
        match node.build(&context) {
            Ok(data) => {
                assert_eq!(data.end_offset, 21);
                assert_eq!(data.is_nesting_started, false);
                assert_eq!(node.base_node.start_offset, 21);
                assert_eq!(node.base_node.end_offset, 42);
                assert_eq!(node.base_node.has_nolinebreak_beginning, true);
                assert_eq!(node.base_node.has_nolinebreak_end, false);
            },
            _ => panic!("Failed to build a node")
        }

        let mut node = CommentNode::create();
        let mut context = BuildContext::new();
        context.template_remain = String::from("{# Here is comment #}World!");
        context.offset = 21;
        match node.build(&context) {
            Ok(data) => {
                assert_eq!(data.end_offset, 20);
                assert_eq!(data.is_nesting_started, false);
                assert_eq!(node.base_node.start_offset, 21);
                assert_eq!(node.base_node.end_offset, 41);
                assert_eq!(node.base_node.has_nolinebreak_beginning, false);
                assert_eq!(node.base_node.has_nolinebreak_end, false);
            },
            _ => panic!("Failed to build a node")
        }
    }
}