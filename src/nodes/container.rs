use crate::engine::{NodeBuildResult, RenderResult};
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

    fn build(&mut self, _template: &String, _offset: usize) -> NodeBuildResult {
        NodeBuildResult::NestedNode(0)
    }

    fn render(&self, parameters: &ParameterStore) -> RenderResult {
        let mut result = String::new();
        for child in &self.base_node.children {
            match child.render(parameters) {
                RenderResult::Ok(r) => {
                    result += r.as_str();
                },
                RenderResult::TemplateError(e) => {
                    return RenderResult::TemplateError(e);
                }
            }
            
        }

        RenderResult::Ok(result)
    }
}