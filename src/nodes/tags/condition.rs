use crate::context::build_context::BuildContext;
use crate::context::render_context::RenderContext;
use crate::engine::{NodeBuildResult, RenderResult};
use crate::expressions::nodes::{Node as ExpressionNode};
use crate::error::template_error::TemplateError;
use crate::nodes::{BaseNode, Node, TAG_START, TAG_END};

const IF_KEYWORD: &str = "if";
const ENDIF_KEYWORD: &str = "endif";

pub struct ConditionNode {
    base_node: BaseNode,
    expressions: Vec<(Box<dyn ExpressionNode>, Box<dyn Node>,)>,
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
        let s = context.template_remain.clone();
        self.base_node.start_offset = context.offset;
        match s.find(TAG_END) {
            Some(end_pos) => NodeBuildResult::NestedNode(end_pos+TAG_END.len()-1),
            None => NodeBuildResult::Error(TemplateError::create(
                context.template.clone(),
                context.offset,
                String::from("Cannot find closing tag."))),
        }
    }

    fn build_if_block_end(&mut self, context: &BuildContext, string: &str) -> NodeBuildResult {
        let s = context.template_remain.clone();
        match s.find(TAG_END) {
            Some(end_pos) => NodeBuildResult::EndOfNode(end_pos+TAG_END.len()-1),
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
        match self.expressions.last_mut() {
            None => { return; },
            Some(e) => { e.1 = child; },
        };
    }

    fn build(&mut self, context: &BuildContext) -> NodeBuildResult {
        let string = strip_chars_before_keyword(&context.template_remain);
        if string.starts_with(IF_KEYWORD) {
            return self.build_if_block_start(context, string);
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

    fn render(&self, _context: &RenderContext) -> RenderResult {
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
                assert_eq!(offset, 22);
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