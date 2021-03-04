use pete_core::engine::Engine;
use pete_core::common::variable::{VariableStore, Variable};

#[test]
fn test_expression_variable() {
    let engine = Engine::new();
    let mut variables = VariableStore::new();
    variables.insert(String::from("user"), Variable::new_from_str("John"));
    match engine.render(String::from("Hello, {{ user }}!"), variables) {
        Ok(string) => assert_eq!(string, String::from("Hello, John!")),
        Err(e) => panic!("Error: {}", &e.message)
    }
}