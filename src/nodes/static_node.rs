use crate::context::build_context::BuildContext;
use crate::context::render_context::RenderContext;
use crate::engine::{NodeBuildResult, RenderResult};
use crate::nodes::{BaseNode, Node, COMMENT_START, EXPRESSION_START, TAG_START};

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
        let mut end_pos = context.template_remain.find(COMMENT_START);
        if end_pos.is_none() {
            end_pos = context.template_remain.find(TAG_START);
        }
        if end_pos.is_none() {
            end_pos = context.template_remain.find(EXPRESSION_START);
        }
        let end_pos = (if end_pos.is_none() { context.template_remain.len() } else { end_pos.unwrap() }) - 1;
        self.base_node.start_offset = context.offset;
        self.base_node.end_offset = context.offset + end_pos;
        self.content = context.template_remain[0..end_pos+1].to_string();
        NodeBuildResult::EndOfNode(end_pos)
    }

    fn is_continuation(&self, _context: &BuildContext) -> bool {
        return false;
    }

    fn render(&self, _context: &RenderContext) -> RenderResult {
        println!("RENDER static {}", self.debug_print());
        return Result::Ok(self.content.clone())
    }


    fn get_base_node(&self) -> &BaseNode {
        return &self.base_node;
    }

    fn debug_name(&self) -> &str {
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
            NodeBuildResult::EndOfNode(offset) => {
                assert_eq!(offset, 12);
            },
            _ => panic!("Failed to build a node")
        }
        match node.render(&RenderContext::new()) {
            Ok(string) => {
                assert_eq!(String::from("Hello, World!"), string);
            },
            Err(e) => panic!("Expected to render a node, but got an error: {}", e)
        }
    }

}