use crate::engine::RenderResult;
use crate::error::Error;
use crate::nodes::{BaseNode, Node};
use crate::parameter::ParameterStore;

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

    pub fn try_create_from_template(template: &String, offset: usize) -> Option<Box<dyn Node>> {
        let substr = template[offset..].to_string();
        if substr.starts_with("{#") || substr.starts_with("{%") || substr.starts_with("{{") {
            None
        } else {
            let mut node = StaticNode::create();
            node.base_node.start_offset = offset;
            Some(Box::from(node))
        }
    }
}

impl Node for StaticNode {
    fn add_child(&mut self, _child: Box<dyn Node>) {
        panic!("Cannot add a child to static node");
    }

    fn build(&mut self, template: &String, offset: usize) -> RenderResult {
        let substr = template[offset..].to_string();
        let mut end_pos = substr.find("{#");
        if end_pos.is_none() {
            end_pos = substr.find("{%");
        }
        if end_pos.is_none() {
            end_pos = substr.find("{{");
        }
        let end_pos = if end_pos.is_none() { substr.len() } else { end_pos.unwrap() };
        self.base_node.end_offset = end_pos;
        self.content = substr[0..self.base_node.end_offset].to_string();
        RenderResult::EndOfNode(self.base_node.end_offset-1)
    }

    fn render(&self, _parameters: &ParameterStore) -> Result<String, Error> {
        Ok(self.content.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nodes_static_render_static_only() {
        let mut node = StaticNode::create();
        let result = node.build(&String::from("Hello, World!"), 0);
        match result {
            RenderResult::EndOfNode(offset) => {
                assert_eq!(offset, 12);
            },
            _ => panic!("Failed to build a node")
        }
        match node.render(&ParameterStore::new()) {
            Ok(string) => {
                assert_eq!(String::from("Hello, World!"), string);
            },
            Err(e) => panic!("Expected to render a node, buy got an error: {}", e)
        }
    }

}