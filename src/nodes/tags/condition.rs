use crate::context::build_context::BuildContext;
use crate::context::render_context::RenderContext;
use crate::engine::{NodeBuildResult, RenderResult};
use crate::expressions;
use crate::expressions::nodes::{Node as ExpressionNode};
use crate::error::template_error::TemplateError;
use crate::nodes::{BaseNode, Node, TAG_START, TAG_END};

const IF_KEYWORD: &str = "if";
const ENDIF_KEYWORD: &str = "endif";

pub struct ConditionNode {
    base_node: BaseNode,
    // Indexes of expressions match to indexes of children nodes
    expressions: Vec<Box<dyn ExpressionNode>>,
}

impl ConditionNode {
    fn create() -> ConditionNode {
        ConditionNode{
            base_node: BaseNode::new(),
            expressions: Vec::new(),
        }
    }

    pub fn try_create_from_template(template: &String) -> Option<Box<dyn Node>> {
        let string = strip_chars_before_keyword(template);
        match string.starts_with(IF_KEYWORD) {
            true => Some(Box::from(ConditionNode::create())),
            false => None
        }
    }

    fn build_if_block_start(&mut self, context: &BuildContext, string: &str) -> NodeBuildResult {
        self.base_node.start_offset = context.offset;
        self.base_node.has_nolinebreak_beginning = &context.template_remain[TAG_START.len()+1..TAG_START.len()+2] == "-";
        let tag_end_pos_rel = match expressions::get_end_offset(String::from(string), TAG_END) {
            Some(end_pos) => end_pos,
            None => {
                return NodeBuildResult::Error(TemplateError::create(
                    context.template.clone(),
                    context.offset,
                    String::from("Cannot find closing tag.")));
            }
        };
        let offset_shift = context.template_remain.len() - string.len();
        let tag_end_pos_abs = tag_end_pos_rel + offset_shift;
        let expr_start_pos = offset_shift;
        let expr_end_pos = tag_end_pos_abs - TAG_END.len() + 1;
        let expr_string = context.template_remain[expr_start_pos..expr_end_pos].to_string();
        match expressions::parse(String::from(expr_string)) {
            Ok(expr_node) => {
                self.expressions.push(expr_node);
                NodeBuildResult::NestedNode(tag_end_pos_abs)
            },
            Err(err) => NodeBuildResult::Error(TemplateError::create(
                context.template.clone(),
                context.offset,
                String::from(format!("An error in the Condition Node. Failed to evaluate an expression: {}", err.message))
            ))
        }
    }

    fn build_if_block_end(&mut self, context: &BuildContext, string: &str) -> NodeBuildResult {
        match expressions::get_end_offset(context.template_remain.clone(), TAG_END) {
            Some(end_pos) => {
                self.base_node.end_offset = context.offset + end_pos;
                NodeBuildResult::EndOfNode(end_pos)
            },
            None => NodeBuildResult::Error(TemplateError::create(
                context.template.clone(),
                context.offset,
                String::from("Cannot find closing tag."))),
        }
    }

}

fn strip_chars_before_keyword(string: &String) -> &str {
    let s1 = match string.strip_prefix(TAG_START) {
        Some(s) => s,
        None => string,
    };
    
    
    match s1.strip_prefix("-") {
        Some(s) => s,
        None => s1,
    }.trim_start_matches(" ")
}

impl Node for ConditionNode {
    fn add_child(&mut self, child: Box<dyn Node>) {
        self.base_node.children.push(child);
    }

    fn build(&mut self, context: &BuildContext) -> NodeBuildResult {
        let string = strip_chars_before_keyword(&context.template_remain);
        if string.starts_with(IF_KEYWORD) {
            return self.build_if_block_start(context, &string[IF_KEYWORD.len()..]);
        } else if string.starts_with(ENDIF_KEYWORD) {
            return self.build_if_block_end(context, string);
        } else {
            return NodeBuildResult::Error(TemplateError::create(
                context.template.clone(),
                context.offset,
                String::from("Unknown keyword. Expected: (if|else|elseif|endif)")));
        }
    }

    fn is_continuation(&self, context: &BuildContext) -> bool {
        let string = strip_chars_before_keyword(&context.template_remain);
        return string.starts_with(ENDIF_KEYWORD);
    }

    fn render(&self, context: &RenderContext) -> RenderResult {
        for (i, expression) in self.expressions.iter().enumerate() {
            let result = match expression.evaluate(context) {
                Ok(variable) => {
                    if variable.get_boolean_value() {
                        let child = match self.base_node.children.get(i) {
                            Some(child) => child,
                            None => {
                                return RenderResult::Err(TemplateError::create(
                                    context.template.clone(),
                                    context.offset,
                                    String::from(format!("An item with index {} not found in children nodes", i))
                                ))
                            }
                        };
                        let r = match child.render(context) {
                            Ok(rendered_string) => RenderResult::Ok(rendered_string),
                            Err(err) => RenderResult::Err(TemplateError::create(
                                context.template.clone(),
                                context.offset,
                                String::from(format!("Failed to evaluate an expression: {}", err.message))
                            )),
                        };
                        Some(r)
                    } else {
                        None
                    }
                },
                Err(err) => Some(RenderResult::Err(TemplateError::create(
                    context.template.clone(),
                    context.offset,
                    String::from(format!("Failed to evaluate an expression: {}", err.message))
                ))),
            };
            match result {
                Some(r) => { return r; },
                None => {},
            }
        }
        RenderResult::Ok(String::new())
    }

    fn has_nolinebreak_end(&self) -> bool {
        self.base_node.has_nolinebreak_end
    }

    fn has_nolinebreak_beginning(&self) -> bool {
        self.base_node.has_nolinebreak_beginning
    }

    fn get_base_node(&self) -> &BaseNode {
        return &self.base_node;
    }

    fn debug_name(&self) -> &str {
        return "condition";
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nodes_tags_condition_try_create_success() {
        let node = ConditionNode::try_create_from_template(&String::from("{% if 4+2 %}test{%endif%}"));
        assert_eq!(node.is_some(), true);
    }

    #[test]
    fn test_nodes_tags_condition_try_create_success_nolinebreak() {
        let node = ConditionNode::try_create_from_template(&String::from("{%- if 4+2 %}test{%endif%}"));
        assert_eq!(node.is_some(), true);
    }


    #[test]
    fn test_nodes_tags_condition_try_create_failure() {
        let node = ConditionNode::try_create_from_template(&String::from("{% for x in a %}"));
        assert_eq!(node.is_none(), true);
    }

    #[test]
    fn test_nodes_tags_condition_render() {
        let mut node = ConditionNode::create();
        let mut context = BuildContext::new();
        context.template_remain = String::from("{% if 4+2 %}test{% endif %}");
        context.offset = 7;
        let result = node.build(&context);
        match result {
            NodeBuildResult::NestedNode(offset) => {
                assert_eq!(offset, 11);
            },
            _ => panic!("Failed to build a node")
        }
        context.template_remain = String::from("{% endif %}");
        context.offset = 15;
        match node.build(&context) {
            NodeBuildResult::EndOfNode(offset) => {
                assert_eq!(offset, 10);
            },
            _ => panic!("Failed to close a node")
        }


        let context = RenderContext::new();
        match node.render(&context) {
            Ok(string) => {
                assert_eq!(String::new(), string);
            },
            Err(e) => panic!("Expected to render a node, but got an error: {}", e)
        }
    }
}