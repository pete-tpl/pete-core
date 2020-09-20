use crate::context::build_context::BuildContext;
use crate::context::render_context::RenderContext;
use crate::engine::{NodeBuildResult, RenderResult};
use crate::nodes::{BaseNode, Node};

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

    fn build(&mut self, _context: &BuildContext) -> NodeBuildResult {
        NodeBuildResult::NestedNode(0)
    }

    fn render(&self, context: &RenderContext) -> RenderResult {
        let mut result = String::new();
        for child in &self.base_node.children {
            result += child.render(&context)?.as_str();
        }

        RenderResult::Ok(result)
    }
}