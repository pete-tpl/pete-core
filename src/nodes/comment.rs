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

    fn build(&mut self, template: &String, offset: usize) -> RenderResult {
        self.base_node.has_nolinebreak_beginning = template[2..3].to_string() == "-";
        let end_pos = template.find("#}");
        match end_pos {
            None => RenderResult::Error(Error::create("Comment is not closed".to_string(), Some(offset))),
            Some(end_pos) => {
                let end_pos_with_tag = end_pos - 1 + "#}".len();
                self.base_node.end_offset = offset + end_pos_with_tag;
                self.base_node.has_nolinebreak_end = template[end_pos-1..end_pos].to_string() == "-";
                self.base_node.start_offset = offset;
                RenderResult::EndOfNode(end_pos_with_tag)
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
    fn test_nodes_static_render_with_static() {
        let mut node = CommentNode::create();
        let result = node.build(&String::from("{# Here is comment #}World!"), 7);
        match result {
            RenderResult::EndOfNode(offset) => {
                assert_eq!(offset, 20);
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

    #[test]
    fn test_nodes_static_nolinebraks() {
        let mut node = CommentNode::create();
        match node.build(&String::from("{#- Here is comment -#}World!"), 21) {
            RenderResult::EndOfNode(offset) => {
                assert_eq!(offset, 22);
                assert_eq!(node.base_node.start_offset, 21);
                assert_eq!(node.base_node.end_offset, 43);
                assert_eq!(node.base_node.has_nolinebreak_beginning, true);
                assert_eq!(node.base_node.has_nolinebreak_end, true);
            },
            _ => panic!("Failed to build a node")
        }

        let mut node = CommentNode::create();
        match node.build(&String::from("{# Here is comment -#}World!"), 21) {
            RenderResult::EndOfNode(offset) => {
                assert_eq!(offset, 21);
                assert_eq!(node.base_node.start_offset, 21);
                assert_eq!(node.base_node.end_offset, 42);
                assert_eq!(node.base_node.has_nolinebreak_beginning, false);
                assert_eq!(node.base_node.has_nolinebreak_end, true);
            },
            _ => panic!("Failed to build a node")
        }

        let mut node = CommentNode::create();
        match node.build(&String::from("{#- Here is comment #}World!"), 21) {
            RenderResult::EndOfNode(offset) => {
                assert_eq!(offset, 21);
                assert_eq!(node.base_node.start_offset, 21);
                assert_eq!(node.base_node.end_offset, 42);
                assert_eq!(node.base_node.has_nolinebreak_beginning, true);
                assert_eq!(node.base_node.has_nolinebreak_end, false);
            },
            _ => panic!("Failed to build a node")
        }

        let mut node = CommentNode::create();
        match node.build(&String::from("{# Here is comment #}World!"), 21) {
            RenderResult::EndOfNode(offset) => {
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