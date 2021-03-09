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

#[test]
fn test_nested_condition() {
    let engine = Engine::new();
    let mut variables = VariableStore::new();
    variables.insert(String::from("myvar"), Variable::new_from_int(2));

    match engine.render(String::from("{% if myvar - 2 %}Hidden{% else %}TEST{% if 1 %}!!!{% endif %} Displayed{% endif %}"), variables) {
        Ok(string) => assert_eq!(string, String::from("TEST!!! Displayed")),
        Err(e) => panic!("Error: {}", &e.message)
    }
}