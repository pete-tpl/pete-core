use crate::engine::{NodeBuildResult, RenderResult};
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

    pub fn try_create_from_template(template: &String) -> Option<Box<dyn Node>> {
        if template.starts_with("{#") || template.starts_with("{%") || template.starts_with("{{") {
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

    fn build(&mut self, template: &String, offset: usize) -> NodeBuildResult {
        let mut end_pos = template.find("{#");
        if end_pos.is_none() {
            end_pos = template.find("{%");
        }
        if end_pos.is_none() {
            end_pos = template.find("{{");
        }
        let end_pos = (if end_pos.is_none() { template.len() } else { end_pos.unwrap() }) - 1;
        self.base_node.end_offset = offset + end_pos;
        self.content = template[0..end_pos+1].to_string();
        self.base_node.start_offset = offset;
        NodeBuildResult::EndOfNode(end_pos)
    }

    fn render(&self, _parameters: &ParameterStore) -> RenderResult {
        RenderResult::Ok(self.content.clone())
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
            NodeBuildResult::EndOfNode(offset) => {
                assert_eq!(offset, 12);
            },
            _ => panic!("Failed to build a node")
        }
        match node.render(&ParameterStore::new()) {
            RenderResult::Ok(string) => {
                assert_eq!(String::from("Hello, World!"), string);
            },
            RenderResult::TemplateError(e) => panic!("Expected to render a node, but got an error: {}", e)
        }
    }

}