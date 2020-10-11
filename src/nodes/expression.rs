use crate::context::build_context::BuildContext;
use crate::context::render_context::RenderContext;
use crate::engine::{NodeBuildResult, RenderResult};
use crate::error::template_error::TemplateError;
use crate::nodes::{BaseNode, Node, EXPRESSION_START, EXPRESSION_END};


pub struct ExpressionNode {
    base_node: BaseNode,
}

impl ExpressionNode {
    fn create() -> ExpressionNode {
        ExpressionNode {
            base_node: BaseNode::new(),
        }
    }

    pub fn try_create_from_template(template: &String) -> Option<Box<dyn Node>> {
        if template.starts_with(EXPRESSION_START) {
            Some(Box::from(ExpressionNode::create()))
        } else {
            None            
        }
    }
}

impl Node for ExpressionNode {
    fn add_child(&mut self, _child: Box<dyn Node>) {
        panic!("Cannot add a child to expression node");
    }

    fn build(&mut self, context: &BuildContext) -> NodeBuildResult {
        self.base_node.has_nolinebreak_beginning = context.template_remain[2..3].to_string() == "-";
        let end_pos = context.template_remain.find(EXPRESSION_END);
        match end_pos {
            None => NodeBuildResult::Error(TemplateError::create(
                context.template.clone(),
                context.offset,
                String::from("Expression is not closed"))),
            Some(end_pos) => {
                let end_pos_with_tag = end_pos - 1 + EXPRESSION_END.len();
                self.base_node.end_offset = context.offset + end_pos_with_tag;
                self.base_node.has_nolinebreak_end = context.template_remain[end_pos-1..end_pos].to_string() == "-";
                self.base_node.start_offset = context.offset;
                NodeBuildResult::EndOfNode(end_pos_with_tag)
            }
        }
    }

    fn render(&self, _context: &RenderContext) -> RenderResult {
        Result::Ok(String::new())
    }

    fn has_nolinebreak_end(&self) -> bool {
        self.base_node.has_nolinebreak_end
    }

    fn has_nolinebreak_beginning(&self) -> bool {
        self.base_node.has_nolinebreak_beginning
    }
}