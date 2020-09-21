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
        let mut previous_no_linebreak_end = false;
        for child in &self.base_node.children {
            let mut child_render_result = child.render(&context)?;
            // Remove a linebreak from beginning of current node if the previous node has a nolinebreak at the end
            if previous_no_linebreak_end {
                child_render_result = match child_render_result.strip_prefix("\n") {
                    Some(r) => String::from(r),
                    None => child_render_result
                };
            }
            // Remove a linebreak from end of currently rendered result if current node has nolinebreak at the beginning
            if child.has_nolinebreak_beginning() {
                result = match result.strip_suffix("\n") {
                    Some(r) => String::from(r),
                    None => result
                };
            }
            result += child_render_result.as_str();
            previous_no_linebreak_end = child.has_nolinebreak_end();
        }

        RenderResult::Ok(result)
    }

    fn has_nolinebreak_end(&self) -> bool {
        self.base_node.has_nolinebreak_end
    }

    fn has_nolinebreak_beginning(&self) -> bool {
        self.base_node.has_nolinebreak_beginning
    }
}