use crate::context::build_context::BuildContext;
use crate::context::render_context::RenderContext;
use crate::engine::{NodeBuildResult, RenderResult};
use crate::error::template_error::TemplateError;
use crate::nodes::{BaseNode, Node};

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
        if template.starts_with("{#") {
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
        self.base_node.has_nolinebreak_beginning = context.template_remain[2..3].to_string() == "-";
        let end_pos = context.template_remain.find("#}");
        match end_pos {
            None => NodeBuildResult::Error(TemplateError::create(
                context.template.clone(),
                context.offset,
                String::from("Comment is not closed"))),
            Some(end_pos) => {
                let end_pos_with_tag = end_pos - 1 + "#}".len();
                self.base_node.end_offset = context.offset + end_pos_with_tag;
                self.base_node.has_nolinebreak_end = context.template_remain[end_pos-1..end_pos].to_string() == "-";
                self.base_node.start_offset = context.offset;
                NodeBuildResult::EndOfNode(end_pos_with_tag)
            }
        }
    }

    fn render(&self, _context: &RenderContext) -> RenderResult {
        RenderResult::Ok(String::new())
    }

    fn has_nolinebreak_end(&self) -> bool {
        self.base_node.has_nolinebreak_end
    }

    fn has_nolinebreak_beginning(&self) -> bool {
        self.base_node.has_nolinebreak_beginning
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nodes_static_render_with_static() {
        let mut node = CommentNode::create();
        let mut context = BuildContext::new();
        context.template_remain = String::from("{# Here is comment #}World!");
        context.offset = 7;
        let result = node.build(&context);
        match result {
            NodeBuildResult::EndOfNode(offset) => {
                assert_eq!(offset, 20);
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
            NodeBuildResult::EndOfNode(offset) => {
                assert_eq!(offset, 22);
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
            NodeBuildResult::EndOfNode(offset) => {
                assert_eq!(offset, 21);
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
            NodeBuildResult::EndOfNode(offset) => {
                assert_eq!(offset, 21);
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
            NodeBuildResult::EndOfNode(offset) => {
                assert_eq!(offset, 20);
                assert_eq!(node.base_node.start_offset, 21);
                assert_eq!(node.base_node.end_offset, 41);
                assert_eq!(node.base_node.has_nolinebreak_beginning, false);
                assert_eq!(node.base_node.has_nolinebreak_end, false);
            },
            _ => panic!("Failed to build a node")
        }
    }
}