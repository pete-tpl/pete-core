use crate::engine::RenderResult;
use crate::error::Error;
use crate::nodes::{BaseNode, Node};
use crate::parameter::ParameterStore;

pub struct ContainerNode {
    base_node: BaseNode,
}

impl ContainerNode {
    pub fn create() -> ContainerNode {
        ContainerNode{
            base_node: BaseNode::new(),
        }
    }

}

impl Node for ContainerNode {
    fn add_child(&mut self, child: Box<dyn Node>) {
        self.base_node.children.push(child);
    }

    fn build(&mut self, _template: &String, _offset: usize) -> RenderResult {
        RenderResult::NestedNode(0)
    }

    fn render(&self, parameters: &ParameterStore) -> Result<String, Error> {
        let mut result = String::new();
        for child in &self.base_node.children {
            result += child.render(parameters)?.as_str();
        }

        Ok(result)
    }
}