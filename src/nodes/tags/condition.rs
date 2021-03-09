use crate::context::build_context::BuildContext;
use crate::context::render_context::RenderContext;
use crate::engine::{NodeBuildResult, RenderResult};
use crate::expressions;
use crate::expressions::nodes::{Node as ExpressionNode};
use crate::expressions::nodes::general::literal::Literal;
use crate::error::template_error::TemplateError;
use crate::nodes::{BaseNode, Node, TAG_START, TAG_END};
use crate::nodes::container::ContainerNode;

const IF_KEYWORD: &str = "if";
const ELSE_KEYWORD: &str = "else";
const ELSEIF_KEYWORD: &str = "elseif";
const ENDIF_KEYWORD: &str = "endif";

pub struct ConditionNode {
    base_node: BaseNode,
    // Indexes of expressions match to indexes of children nodes
    expressions: Vec<Box<dyn ExpressionNode>>,
}

// Parses an expression from non-parsed template
fn parse_expression(full_template: &String, nonparsed_template: &String) -> Result<(String, usize), String> {
    let tag_end_pos_rel = match expressions::get_end_offset(nonparsed_template, TAG_END) {
        Some(end_pos) => end_pos,
        None => {
            return Err(String::from("Cannot find closing tag."));
        }
    };

    let offset_shift = full_template.len() - nonparsed_template.len();
    let tag_end_pos_abs = tag_end_pos_rel + offset_shift;
    let expr_start_pos = offset_shift;
    let expr_end_pos = tag_end_pos_abs - TAG_END.len() + 1;
    let expr_string = full_template[expr_start_pos..expr_end_pos].to_string();
    Ok((expr_string, tag_end_pos_abs))
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

    fn build_block_if(&mut self, context: &BuildContext, string: &String) -> NodeBuildResult {
        self.base_node.start_offset = context.offset;
        self.base_node.has_nolinebreak_beginning = &context.template_remain[TAG_START.len()+1..TAG_START.len()+2] == "-";
        let (expr_string, tag_end_pos_abs) = match parse_expression(&context.template_remain, string) {
            Ok(s) => s,
            Err(s) => return NodeBuildResult::Error(TemplateError::create(
                context.template.clone(),
                context.offset,
                s))
        };

        match expressions::parse(String::from(expr_string)) {
            Ok(expr_node) => {
                self.expressions.push(expr_node);
                let mut container = ContainerNode::create();
                let mut container_base_node = container.get_base_node_mut();
                container_base_node.start_offset = context.offset + tag_end_pos_abs + 1;
                container_base_node.end_offset = container_base_node.start_offset;
                self.base_node.children.push(Box::from(container));
                NodeBuildResult::NestedNode(tag_end_pos_abs)
            },
            Err(err) => NodeBuildResult::Error(TemplateError::create(
                context.template.clone(),
                context.offset,
                String::from(format!("An error in the Condition Node. Failed to evaluate an expression: {}", err.message))
            ))
        }
    }

    fn build_if_block_else(&mut self, context: &BuildContext, string: &String) -> NodeBuildResult {
        let (expr_string, tag_end_pos_abs) = match parse_expression(&context.template_remain, string) {
            Ok(s) => s,
            Err(s) => return NodeBuildResult::Error(TemplateError::create(
                context.template.clone(),
                context.offset,
                s))
        };

        if expr_string.trim_matches(' ').len() > 0 {
            return NodeBuildResult::Error(TemplateError::create(
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
        self.base_node.children.push(Box::from(container));
        NodeBuildResult::NestedNode(tag_end_pos_abs)
    }

    fn build_block_end(&mut self, context: &BuildContext) -> NodeBuildResult {
        match expressions::get_end_offset(&context.template_remain, TAG_END) {
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

    // Renders a condition with index "index"
    fn render_conditional_block(&self, index: usize, context: &RenderContext) -> RenderResult {
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

// Returns two strings: a keyword (if, elseif, etc) and a remain AFTER keyword
// e.g. get_keyword("{% if 1 + 1 %}") => ("if", " 1 + 1 %}")
fn get_keyword(string: &String) -> (&str, String) {
    let s = match string.strip_prefix(TAG_START) {
        Some(m) => m,
        None => return ("", string.clone()),
    };
    let s = match s.strip_prefix('-') {
        Some(m) => m,
        None => s,
    }.trim_start_matches(' ');
    let endpos = match s.find(|c| !char::is_alphabetic(c)) {
        Some(p) => p,
        None => s.len() - 1,
    };
    (&s[..endpos], String::from(&s[endpos..]))
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
        let (keyword, remain) = get_keyword(&context.template_remain);
        match keyword {
            IF_KEYWORD|ELSEIF_KEYWORD => self.build_block_if(context, &remain),
            ELSE_KEYWORD => self.build_if_block_else(context, &remain),
            ENDIF_KEYWORD => self.build_block_end(context),
            _ => NodeBuildResult::Error(TemplateError::create(
                context.template.clone(),
                context.offset,
                String::from("Unknown keyword. Expected: (if|else|elseif|endif)"))),
        }
    }

    fn is_continuation(&self, context: &BuildContext) -> bool {
        let keyword = get_keyword(&context.template_remain).0;
        ELSEIF_KEYWORD == keyword || ELSE_KEYWORD == keyword || ENDIF_KEYWORD == keyword
    }

    fn render(&self, context: &RenderContext) -> RenderResult {
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

    fn get_base_node(&self) -> &BaseNode {
        return &self.base_node;
    }

    fn get_base_node_mut(&mut self) -> &mut BaseNode {
        return &mut self.base_node;
    }

    fn debug_name(&self) -> &str {
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
            NodeBuildResult::NestedNode(offset) => {
                assert_eq!(offset, 11);
            },
            _ => panic!("Failed to build a node")
        }

        node.add_child(Box::from(StaticNode::try_create_from_template(&String::from("test")).unwrap()));

        context.template_remain = String::from("{% endif %}");
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
            NodeBuildResult::NestedNode(offset) => {
                assert_eq!(offset, 17);
            },
            NodeBuildResult::Error(e) => panic!("Failed to build a node {}", e.message.clone()),
            _ => {}
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
            NodeBuildResult::NestedNode(offset) => {
                assert_eq!(offset, 9);
            },
            NodeBuildResult::Error(e) => panic!("Failed to build a node {}", e.message.clone()),
            _ => {}
        };
    }
}