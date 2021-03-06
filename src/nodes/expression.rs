use crate::context::build_context::BuildContext;
use crate::context::render_context::RenderContext;
use crate::engine::{NodeBuildResult, RenderResult};
use crate::error::template_error::TemplateError;
use crate::expressions as expression_mod;
use crate::expressions::nodes as expression_nodes;
use crate::expressions::nodes::general::literal::Literal;
use crate::nodes::{BaseNode, Node, EXPRESSION_START, EXPRESSION_END};

pub struct ExpressionNode {
    base_node: BaseNode,
    build_context: BuildContext,
    expression_node: Box<dyn expression_nodes::Node>,
}

impl ExpressionNode {
    fn create() -> ExpressionNode {
        ExpressionNode {
            base_node: BaseNode::new(),
            build_context: BuildContext::new(),
            expression_node: Box::new(Literal::new_from_str("")),
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
        self.build_context = context.clone();
        match expression_mod::get_end_offset(&context.template_remain, EXPRESSION_END) {
            None => NodeBuildResult::Error(TemplateError::create(
                context.template.clone(),
                context.offset,
                String::from("Expression is not closed"))),
            Some(end_pos_with_tag) => {
                let end_pos = end_pos_with_tag - EXPRESSION_END.len();
                self.base_node.end_offset = context.offset + end_pos_with_tag;
                self.base_node.has_nolinebreak_end = context.template_remain[end_pos-1..end_pos].to_string() == "-";
                self.base_node.start_offset = context.offset;
                let expression_string = context.template_remain[EXPRESSION_START.len()..end_pos].to_string();
                match expression_mod::parse(expression_string) {
                    Ok(expr_node) => {
                        self.expression_node = expr_node;
                    },
                    Err(err) => {
                        return NodeBuildResult::Error(TemplateError::create(
                            self.build_context.template.clone(),
                            self.build_context.offset,
                            String::from(format!("Failed to build an expression: {}", err.message))
                        ));
                    }
                };
                NodeBuildResult::EndOfNode(end_pos_with_tag)
            }
        }
    }

    fn is_continuation(&self, _context: &BuildContext) -> bool {
        return false;
    }

    fn render(&self, context: &RenderContext) -> RenderResult {
        println!("RENDER expression {}", self.debug_print());
        match self.expression_node.evaluate(&context) {
            Ok(parameter) => RenderResult::Ok(parameter.as_string()),
            Err(err) => RenderResult::Err(TemplateError::create(
                self.build_context.template.clone(),
                self.build_context.offset,
                String::from(format!("Failed to evaluate an expression: {}", err.message))
            )),
        }
        
    }

    fn get_base_node(&self) -> &BaseNode {
        return &self.base_node;
    }

    fn debug_name(&self) -> &str {
        return "expression";
    }
}