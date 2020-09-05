// use crate::functions::Function;
use crate::error::Error;
use crate::parameter::ParameterStore;

pub struct Engine {
    // functions: Vec<Function>
}

impl Engine {
    pub fn new() -> Engine {
        Engine {

        }
    }

    pub fn render(&self, template: String, _parameters: ParameterStore) -> Result<String, Error> {
        Ok(template)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render() {
        let engine = Engine::new();
        let result = engine.render(String::from("Hello, World!"), ParameterStore::new());
        assert_eq!(result.unwrap(), "Hello, World!");
    }
}