use crate::context::build_context::BuildContext;
use crate::context::render_context::RenderContext;
use crate::engine::{NodeBuildData, NodeBuildResult, RenderResult};
use crate::nodes::{BaseNode, Node};

use derive_macro::HasBaseNode;

#[derive(HasBaseNode)]
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
        Ok(NodeBuildData::new(0, false, false))
    }

    fn is_continuation(&self, _context: &BuildContext) -> bool {
        return false;
    }

    fn is_control_node(&self) -> bool {
        return false;
    }

    fn render(&self, context: &mut RenderContext) -> RenderResult {
        let mut result = String::new();
        let mut previous_no_linebreak_end = self.base_node.has_nolinebreak_beginning;
        for child in &self.base_node.children {
            if child.has_nolinebreak_beginning() {
                result = match result.strip_suffix("\n") {
                    Some(r) => String::from(r),
                    None => result
                };
            }

            let mut child_render_result = child.render(context)?;
            if child.get_base_node().has_nolinebreak_end {
                child_render_result = match child_render_result.strip_suffix("\n") {
                    Some(r) => String::from(r),
                    None => child_render_result
                };
            }
            if previous_no_linebreak_end {
                child_render_result = match child_render_result.strip_prefix("\n") {
                    Some(r) => String::from(r),
                    None => child_render_result
                };
            }

            result += child_render_result.as_str();
            previous_no_linebreak_end = child.has_nolinebreak_end();
        }

        RenderResult::Ok(result)
    }

    fn get_name(&self) -> &str {
        return "container";
    }
}