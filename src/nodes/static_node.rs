use crate::engine::RenderResult;
use crate::error::Error;
use crate::nodes::Node;

pub struct StaticNode {

}

impl StaticNode {
    fn create() -> StaticNode {
        StaticNode{}
    }

    pub fn try_create_from_template(template: &String, offset: usize) -> Option<Box<dyn Node>> {
        let substr = template[offset..].to_string();
        if substr.starts_with("{#") || substr.starts_with("{%") || substr.starts_with("{{") {
            None
        } else {
            Some(Box::from(StaticNode::create()))
        }
    }
}

impl Node for StaticNode {
    fn render(&self, template: &String, offset: usize) -> Result<RenderResult, Error> {
        let substr = template[offset..].to_string();
        let mut end_pos = substr.find("{#");
        if end_pos.is_none() {
            end_pos = substr.find("{%");
        }
        if end_pos.is_none() {
            end_pos = substr.find("{{");
        }
        let end_pos = if end_pos.is_none() { substr.len() } else { end_pos.unwrap() };
        Ok(RenderResult::EndOfNode(substr[0..end_pos].to_string(), end_pos-1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nodes_static_render_static_only() {
        let node = StaticNode::create();
        let result = node.render(&String::from("Hello, World!"), 0);
        match result {
            Ok(result) => {
                match result {
                    RenderResult::EndOfNode(string, offset) => {
                        assert_eq!(String::from("Hello, World!"), string);
                        assert_eq!(offset, 12);
                    },
                    _ => panic!("Unexpected node")
                }
            }
            Err(_) => panic!("Expected OK, but got err")
        }
        
        // let engine = Engine::new();
        // let result = engine.render(String::from("Hello, World!"), ParameterStore::new());
        // assert_eq!(result.unwrap(), "Hello, World!");
    }

}