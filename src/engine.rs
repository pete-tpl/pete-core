// use crate::functions::Function;
use crate::error::Error;
use crate::parameter::ParameterStore;

use crate::nodes::NodeCreator;
use crate::nodes::static_node::StaticNode;

const NODE_CREATORS: [NodeCreator; 1] = [
    StaticNode::try_create_from_template
];

pub struct Engine {
    // functions: Vec<Function>
}

pub enum RenderResult {
    EndOfNode(String, usize),
    NestedNode(String, usize)
}

impl Engine {
    pub fn new() -> Engine {
        Engine {

        }
    }

    pub fn render(&self, template: String, _parameters: ParameterStore) -> Result<String, Error> {
        let template_len = template.len();
        let mut cursor = 0;
        let mut rendered_template = String::new();
        while cursor < template_len {
            for node_creator in NODE_CREATORS.iter() {
                match node_creator(&template, cursor) {
                    Some(b) => {
                        match b.render(&template, cursor) {
                            Ok(render_result) => {
                                match render_result {
                                    RenderResult::EndOfNode(string, offset) => {
                                        rendered_template += string.as_str();
                                        cursor += offset + 1;
                                    },
                                    _ => {
                                        return Err(Error::create("Rendering results except RenderResult::EndOfNode are not implemented".to_string()));
                                    }
                                }
                            },
                            Err(error) => {
                                return Err(error);
                            }
                        };
                    },
                    _ => {
                        return Err(Error::create("Cannot recognize a node".to_string()));
                    }
                };
            }
        }
        Ok(rendered_template)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_render_static_only() {
        let engine = Engine::new();
        let result = engine.render(String::from("Hello, World!"), ParameterStore::new());
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), "Hello, World!");
    }

    #[test]
    fn test_engine_render_static_and_comment() {
        let engine = Engine::new();
        let result = engine.render(String::from("Hello, World!{# Some comment here #}Nice to meet you."), ParameterStore::new());
        assert_eq!(result.is_err(), true);
        assert_eq!(result.unwrap_err().message, "Cannot recognize a node");
    }
}