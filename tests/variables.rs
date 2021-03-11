use std::fs;

use pete_core::engine::Engine;
use pete_core::common::variable::{VariableStore, Variable};

fn read_test_files(input: &str, output: &str) -> (String, String) {
    let input = match fs::read_to_string(input) {
        Ok(s) => s,
        Err(_) => panic!("Cannot read input file: {}", input),
    };
    let output = match fs::read_to_string(output) {
        Ok(s) => s,
        Err(_) => panic!("Cannot read output file: {}", output),
    };
    return (input, output);
}

#[test]
fn test_variables_simple() {
    let engine = Engine::new();
    let mut variables = VariableStore::new();
    variables.insert(String::from("user"), Variable::new_from_str("John"));
    let (input, output) = read_test_files("tests/templates/variables/simple.input.twig", "tests/templates/variables/simple.output.txt");
    match engine.render(input, variables) {
        Ok(string) => assert_eq!(string, output),
        Err(e) => panic!("Error: {}", &e.message)
    }
}