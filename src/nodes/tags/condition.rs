use crate::context::build_context::BuildContext;
use crate::context::render_context::RenderContext;
use crate::engine::{NodeBuildResult, NodeBuildData, RenderResult};
use crate::expressions;
use crate::expressions::nodes::{Node as ExpressionNode};
use crate::expressions::nodes::general::literal::Literal;
use crate::error::template_error::TemplateError;
use crate::nodes::{BaseNode, Node, TAG_START, TAG_END};
use crate::nodes::container::ContainerNode;

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

/// Parses an expression from non-parsed template
/// 
/// Returns:
/// - an Expression string
/// - end offset of the tag
/// - if expression as a no linebreak character at the end
/// 
/// # Examples
/// 
/// ```ignore
/// assert_eq!(Ok(("2+3 ", 11, false)), parse_expression("{% if 2+3 %}A{%endif%}", " 2+3 %}A{%endif%}"));
/// assert_eq!(Ok(("2+3 ", 12, true)), parse_expression("{% if 2+3 -%}A{%endif%}", " 2+3 -%}A{%endif%}"));
/// ```
fn parse_expression(full_template: &String, nonparsed_template: &String) -> Result<(String, usize, bool), String> {
    let tag_end_pos_rel = match expressions::get_end_offset(nonparsed_template, TAG_END) {
        Some(end_pos) => end_pos,
        None => {
            return Err(String::from("Cannot find closing tag."));
        }
    };
    let offset_shift = full_template.len() - nonparsed_template.len();
    let tag_end_pos_abs = tag_end_pos_rel + offset_shift;
    let expr_start_pos = offset_shift;
    let (expr_end_pos, no_linebreak_end) = {
        let end_pos = tag_end_pos_abs - TAG_END.len();
        let str_before_end_tag = nonparsed_template[..tag_end_pos_rel - TAG_END.len() + 1].to_string();
        match str_before_end_tag.ends_with('-') {
            true => (end_pos - 1, true),
            false => (end_pos, false),
        }
    };
    let expr_string = full_template[expr_start_pos..expr_end_pos].to_string();
    Ok((expr_string, tag_end_pos_abs, no_linebreak_end))
}

impl ConditionNode {
    fn create() -> ConditionNode {
        ConditionNode{
            base_node: BaseNode::new(),
            expressions: Vec::new(),
        }
    }

    pub fn try_create_from_template(template: &String) -> Option<Box<dyn Node>> {
        match get_keyword(template).0 {
            IF_KEYWORD => Some(Box::from(ConditionNode::create())),
            _ => None
        }
    }

    fn build_block_if(&mut self, context: &BuildContext, string: &String, has_nolinebreak_beginning: bool) -> NodeBuildResult {
        self.base_node.has_nolinebreak_beginning = has_nolinebreak_beginning;
        self.base_node.start_offset = context.offset;
        let (expr_string, tag_end_pos_abs, has_nolinebreak_end) = match parse_expression(&context.template_remain, string) {
            Ok(s) => s,
            Err(s) => return Err(TemplateError::create(
                context.template.clone(),
                context.offset,
                s))
        };

        match expressions::parse(String::from(expr_string)) {
            Ok(expr_node) => {
                self.expressions.push(expr_node);
                let mut container = ContainerNode::create();
                let mut container_base_node = container.get_base_node_mut();
                container_base_node.has_nolinebreak_beginning = has_nolinebreak_beginning;
                container_base_node.has_nolinebreak_end = has_nolinebreak_end;
                container_base_node.start_offset = context.offset + tag_end_pos_abs + 1;
                container_base_node.end_offset = container_base_node.start_offset;
                self.base_node.children.push(Box::from(container));
                Ok(NodeBuildData::new(tag_end_pos_abs, true, has_nolinebreak_end))
            },
            Err(err) => Err(TemplateError::create(
                context.template.clone(),
                context.offset,
                String::from(format!("An error in the Condition Node. Failed to evaluate an expression: {}", err.message))
            ))
        }
    }

    fn build_if_block_else(&mut self, context: &BuildContext, string: &String, has_nolinebreak_beginning: bool) -> NodeBuildResult {
        match self.get_children_mut().last_mut() { // TODO: check if is duplicate
            Some(child) => {
                child.get_base_node_mut().has_nolinebreak_end = has_nolinebreak_beginning;
            },
            None => {},
        };
        let (expr_string, tag_end_pos_abs, has_nolinebreak_end) = match parse_expression(&context.template_remain, string) {
            Ok(s) => s,
            Err(s) => return Err(TemplateError::create(
                context.template.clone(),
                context.offset,
                s))
        };

        let expr_remain = expr_string.trim_matches(' ');
        let expr_remain = match expr_remain.strip_prefix('-') {
            Some(s) => s,
            None => expr_remain,
        };

        if expr_remain.len() > 0 {
            return Err(TemplateError::create(
                context.template.clone(),
                context.offset,
                String::from(format!("An error in the Condition Node. Unexpected characters in ELSE block: {}", expr_string))
            ))
        }

        self.expressions.push(Box::from(Literal::new_from_bool(true)));
        let mut container = ContainerNode::create();
        let mut container_base_node = container.get_base_node_mut();
        container_base_node.start_offset = context.offset + tag_end_pos_abs + 1;
        container_base_node.end_offset = container_base_node.start_offset;
        container_base_node.has_nolinebreak_beginning = has_nolinebreak_beginning;
        container_base_node.has_nolinebreak_end = has_nolinebreak_end;
        self.base_node.children.push(Box::from(container));
        Ok(NodeBuildData::new(tag_end_pos_abs, true, has_nolinebreak_end))
    }

    fn build_block_end(&mut self, context: &BuildContext, has_nolinebreak_beginning: bool) -> NodeBuildResult {
        match self.get_children_mut().last_mut() { // TODO: check if is duplicate
            Some(child) => {
                child.get_base_node_mut().has_nolinebreak_end = has_nolinebreak_beginning;
            },
            None => {},
        };
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
        context.previous_has_nolinebreak_end = match self.base_node.children.get(0) {
            Some(child) => child.get_base_node().has_nolinebreak_beginning,
            None => false,
        };
        {
            let (is_last_static, next_has_nolinebreak_beginning) = match self.base_node.children.last() {
                Some(child) => (child.is_static(), child.get_base_node().has_nolinebreak_end),
                None => (false, false),
            };
            context.next_has_nolinebreak_beginning = next_has_nolinebreak_beginning;
        }

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

/// Returns:
/// - a keyword (if, elseif, etc)
/// - a remain AFTER keyword
/// - if the parsed tag has nolinebreak char
/// 
/// # Examples
/// 
/// ```ignore
/// assert_eq!(get_keyword("{% if 1 + 1 %}"), ("if", " 1 + 1 %}", false));
/// assert_eq!(get_keyword("{%- if 1 + 1 %}"), ("if", " 1 + 1 %}", true));
/// ```
fn get_keyword(string: &String) -> (&str, String, bool) {
    let s = match string.strip_prefix(TAG_START) {
        Some(m) => m,
        None => return ("", string.clone(), false),
    };
    let (s, has_nolinebreak_beginning) = match s.strip_prefix('-') {
        Some(m) => (m, true),
        None => (s, false),
    };
    let s =  s.trim_start_matches(' ');
    let endpos = match s.find(|c| !char::is_alphabetic(c)) {
        Some(p) => p,
        None => s.len() - 1,
    };
    (&s[..endpos], String::from(&s[endpos..]), has_nolinebreak_beginning)
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
        let (keyword, remain, has_nolinebreak_beginning) = get_keyword(&context.template_remain);
        match keyword {
            IF_KEYWORD|ELSEIF_KEYWORD => self.build_block_if(context, &remain, has_nolinebreak_beginning),
            ELSE_KEYWORD => self.build_if_block_else(context, &remain, has_nolinebreak_beginning),
            ENDIF_KEYWORD => self.build_block_end(context, has_nolinebreak_beginning),
            _ => Err(TemplateError::create(
                context.template.clone(),
                context.offset,
                String::from("Unknown keyword. Expected: (if|else|elseif|endif)"))),
        }
    }

    fn is_continuation(&self, context: &BuildContext) -> bool {
        let keyword = get_keyword(&context.template_remain).0;
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