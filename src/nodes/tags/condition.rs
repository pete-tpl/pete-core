use crate::context::build_context::BuildContext;
use crate::context::render_context::RenderContext;
use crate::engine::{NodeBuildResult, NodeBuildData, RenderResult};
use crate::expressions;
use crate::expressions::nodes::{Node as ExpressionNode};
use crate::expressions::nodes::general::literal::Literal;
use crate::error::template_error::TemplateError;
use crate::nodes::{BaseNode, Node};
use crate::nodes::tags::TAG_END;
use crate::nodes::container::ContainerNode;
use crate::parsers::expression_parser::{ parse_expression_string, ParseExpressionStringResult };
use crate::parsers::tag_parser::{get_keyword, GetKeywordResult};

use derive_macro::HasBaseNode;

const IF_KEYWORD: &str = "if";
const ELSE_KEYWORD: &str = "else";
const ELSEIF_KEYWORD: &str = "elseif";
const ENDIF_KEYWORD: &str = "endif";

#[derive(HasBaseNode)]
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
        let keyword = match get_keyword(template) {
            Some(r) => r.keyword,
            None => return None,
        };
        match keyword {
            IF_KEYWORD => Some(Box::from(ConditionNode::create())),
            _ => None
        }
    }

    fn build_block_if(&mut self, context: &BuildContext, get_keyword_result: &GetKeywordResult) -> NodeBuildResult {
        self.base_node.start_offset = context.offset;
        let parsed_expression = match parse_expression_string(&get_keyword_result.remain,
                get_keyword_result.end_pos, TAG_END) {
            Ok(s) => Ok(s) as Result<ParseExpressionStringResult, TemplateError>,
            Err(s) => return Err(TemplateError::create(
                context.template.clone(),
                context.offset,
                s))
        }?;

        match expressions::parse(parsed_expression.expression_string.clone()) {
            Ok(expr_node) => {
                self.expressions.push(expr_node);
                let mut container = ContainerNode::create();
                let mut container_base_node = container.get_base_node_mut();
                container_base_node.has_nolinebreak_beginning = get_keyword_result.has_nolinebreak_beginning;
                container_base_node.has_nolinebreak_end = parsed_expression.has_nolinebreak_end;
                container_base_node.start_offset = context.offset + parsed_expression.end_offset + 1;
                container_base_node.end_offset = container_base_node.start_offset;
                self.base_node.children.push(Box::from(container));
                Ok(NodeBuildData::new(parsed_expression.end_offset, true, parsed_expression.has_nolinebreak_end))
            },
            Err(err) => Err(TemplateError::create(
                context.template.clone(),
                context.offset,
                String::from(format!("An error in the Condition Node. Failed to evaluate an expression: {}", err.message))
            ))
        }
    }

    fn build_if_block_else(&mut self, context: &BuildContext, get_keyword_result: &GetKeywordResult) -> NodeBuildResult {
        let parsed_expression = match parse_expression_string(&get_keyword_result.remain, get_keyword_result.end_pos, TAG_END) {
            Ok(s) => Ok(s) as Result<ParseExpressionStringResult, TemplateError>,
            Err(s) => return Err(TemplateError::create(
                context.template.clone(),
                context.offset,
                s))
        }?;

        let expr_remain = parsed_expression.expression_string.trim_matches(' ');
        let expr_remain = match expr_remain.strip_prefix('-') {
            Some(s) => s,
            None => expr_remain,
        };

        if expr_remain.len() > 0 {
            return Err(TemplateError::create(
                context.template.clone(),
                context.offset,
                String::from(format!("An error in the Condition Node. Unexpected characters in ELSE block: {}", parsed_expression.expression_string))
            ))
        }

        self.expressions.push(Box::from(Literal::new_from_bool(true)));
        let mut container = ContainerNode::create();
        let mut container_base_node = container.get_base_node_mut();
        container_base_node.start_offset = context.offset + parsed_expression.end_offset + 1;
        container_base_node.end_offset = container_base_node.start_offset;
        container_base_node.has_nolinebreak_beginning = get_keyword_result.has_nolinebreak_beginning;
        container_base_node.has_nolinebreak_end = parsed_expression.has_nolinebreak_end;
        self.base_node.children.push(Box::from(container));
        Ok(NodeBuildData::new(parsed_expression.end_offset, true, parsed_expression.has_nolinebreak_end))
    }

    fn build_block_end(&mut self, context: &BuildContext) -> NodeBuildResult {
        match expressions::get_end_offset(&context.template_remain, TAG_END) {
            Some(end_pos) => {
                let has_nolinebreak_end = context.template_remain[..end_pos-TAG_END.len()+1].ends_with('-');
                self.base_node.end_offset = context.offset + end_pos;
                self.base_node.has_nolinebreak_end = has_nolinebreak_end;
                Ok(NodeBuildData::new(end_pos, false, has_nolinebreak_end))
            },
            None => Err(TemplateError::create(
                context.template.clone(),
                context.offset,
                String::from("Cannot find closing tag."))),
        }
    }

    // Renders a condition with index "index"
    fn render_conditional_block(&self, index: usize, context: &mut RenderContext) -> RenderResult {
        let child = match self.base_node.children.get(index) {
            Some(child) => child,
            None => {
                return RenderResult::Err(TemplateError::create(
                    context.template.clone(),
                    context.offset,
                    String::from(format!("An item with index {} not found in children nodes", index))
                ));
            }
        };

        match child.render(context) {
            Ok(rendered_string) => RenderResult::Ok(rendered_string),
            Err(err) => RenderResult::Err(TemplateError::create(
                context.template.clone(),
                context.offset,
                String::from(format!("Failed to evaluate an expression: {}", err.message))
            )),
        }
    }
}

impl Node for ConditionNode {
    fn add_child(&mut self, child: Box<dyn Node>) {
        match self.base_node.children.last_mut() {
            None => {},
            Some(c) => {
                c.get_base_node_mut().set_end_offset(child.get_base_node().end_offset);
                c.add_child(child);
            },
        }
    }

    fn build(&mut self, context: &BuildContext) -> NodeBuildResult {
        let result = match get_keyword(&context.template_remain) {
            Some(r) => Ok(r),
            None => Err(TemplateError::create(
                context.template.clone(),
                context.offset,
                String::from("Unknown keyword. Expected: (if|else|elseif|endif)"))),
        }?;

        match self.get_children_mut().last_mut() {
            Some(child) => child.get_base_node_mut().has_nolinebreak_end = result.has_nolinebreak_beginning,
            None => self.get_base_node_mut().has_nolinebreak_beginning = result.has_nolinebreak_beginning,
        };
        match result.keyword {
            IF_KEYWORD|ELSEIF_KEYWORD => self.build_block_if(context, &result),
            ELSE_KEYWORD => self.build_if_block_else(context, &result),
            ENDIF_KEYWORD => self.build_block_end(context),
            _ => Err(TemplateError::create(
                context.template.clone(),
                context.offset,
                String::from("Unknown keyword. Expected: (if|else|elseif|endif)"))),
        }
    }

    fn is_continuation(&self, context: &BuildContext) -> bool {
        let keyword = match get_keyword(&context.template_remain) {
            Some(r) => r.keyword,
            None => "",
        };
        ELSEIF_KEYWORD == keyword || ELSE_KEYWORD == keyword || ENDIF_KEYWORD == keyword
    }

    fn render(&self, context: &mut RenderContext) -> RenderResult {
        for (i, expression) in self.expressions.iter().enumerate() {
            let result = match expression.evaluate(context) {
                Ok(variable) => {
                    if variable.get_boolean_value() {
                        Some(self.render_conditional_block(i, context))
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

    fn get_name(&self) -> &str {
        return "condition";
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::nodes::static_node::StaticNode;

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
        let result = node.build(&context);
        match result {
            Ok(data) => {
                assert_eq!(data.end_offset, 11);
                assert_eq!(data.is_nesting_started, true);
            },
            _ => panic!("Failed to build a node")
        }

        node.add_child(Box::from(StaticNode::try_create_from_template(&String::from("test")).unwrap()));

        context.template_remain = String::from("{% endif %}");
        match node.build(&context) {
            Ok(data) => {
                assert_eq!(data.end_offset, 10);
                assert_eq!(data.is_nesting_started, false);
            },
            _ => panic!("Failed to close a node")
        }


        let mut context = RenderContext::new();
        match node.render(&mut context) {
            Ok(string) => {
                assert_eq!(String::new(), string);
            },
            Err(e) => panic!("Expected to render a node, but got an error: {}", e)
        }
    }

    #[test]
    fn test_nodes_tags_condition_elseif_build() {
        let mut node = ConditionNode::create();
        let mut context = BuildContext::new();
        context.template_remain = String::from("{% elseif \"abc\" %}test2{% endif %}");
        if !node.is_continuation(&context) {
            panic!("Expected: is_continuation = FALSE, got: TRUE")
        }

        let result = node.build(&context);
        match result {
            Ok(data) => {
                assert_eq!(data.end_offset, 17);
                assert_eq!(data.is_nesting_started, true);
            },
            Err(e) => panic!("Failed to build a node {}", e.message.clone()),
        };
    }

    #[test]
    fn test_nodes_tags_condition_else_build() {
        let mut node = ConditionNode::create();
        let mut context = BuildContext::new();
        context.template_remain = String::from("{% else %}test2{% endif %}");
        if !node.is_continuation(&context) {
            panic!("Expected: is_continuation = FALSE, got: TRUE")
        }

        let result = node.build(&context);
        match result {
            Ok(data) => {
                assert_eq!(data.end_offset, 9);
                assert_eq!(data.is_nesting_started, true);
            },
            Err(e) => panic!("Failed to build a node {}", e.message.clone()),
        };
    }
}