use crate::engine::RenderResult;
use crate::error::Error;
use crate::nodes::{BaseNode, Node};
use crate::parameter::ParameterStore;

pub struct CommentNode {
    base_node: BaseNode,
}

impl CommentNode {
    fn create() -> CommentNode {
        CommentNode{
            base_node: BaseNode::new(),
        }
    }

    pub fn try_create_from_template(template: &String, offset: usize) -> Option<Box<dyn Node>> {
        let substr = template[offset..].to_string();
        if substr.starts_with("{#") {
            let mut node = CommentNode::create();
            node.base_node.start_offset = offset;
            Some(Box::from(node))
        } else {
            None
        }
    }
}

impl Node for CommentNode {
    fn add_child(&mut self, _child: Box<dyn Node>) {
        panic!("Cannot add a child to comment node");
    }

    fn build(&mut self, template: &String, offset: usize) -> RenderResult {
        let substr = template[offset..].to_string();
        let end_pos = substr.find("#}");
        match end_pos {
            None => RenderResult::Error(Error::create("Comment is not closed".to_string(), Some(offset))),
            Some(pos) => {
                self.base_node.end_offset = offset + pos + "#}".len();
                RenderResult::EndOfNode(self.base_node.end_offset-1)
            }
        }
    }

    fn render(&self, _parameters: &ParameterStore) -> Result<String, Error> {
        Ok(String::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nodes_static_render_static_only() {
        let mut node = CommentNode::create();
        let result = node.build(&String::from("Hello, {# Here is comment #}World!"), 7);
        match result {
            RenderResult::EndOfNode(offset) => {
                assert_eq!(offset, 27);
            },
            _ => panic!("Failed to build a node")
        }
        match node.render(&ParameterStore::new()) {
            Ok(string) => {
                assert_eq!(String::new(), string);
            },
            Err(e) => panic!("Expected to render a node, but got an error: {}", e)
        }
    }

}