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
fn test_conditions_simple() {
    let engine = Engine::new();
    let mut variables = VariableStore::new();
    variables.insert(String::from("myvar"), Variable::new_from_int(2));
    let (input, output) = read_test_files("tests/templates/conditions/simple.input.twig", "tests/templates/conditions/simple.output.txt");
    match engine.render(input, variables) {
        Ok(string) => assert_eq!(string, output),
        Err(e) => panic!("Error: {}", &e.message)
    }
}

#[test]
fn test_conditions_nolinebreak() {
    let engine = Engine::new();
    let mut variables = VariableStore::new();
    variables.insert(String::from("myvar"), Variable::new_from_int(2));
    let (input, output) = read_test_files("tests/templates/conditions/nolinebreak.input.twig", "tests/templates/conditions/nolinebreak.output.txt");
    match engine.render(input, variables) {
        Ok(string) => assert_eq!(string, output),
        Err(e) => panic!("Error: {}", &e.message)
    }
}