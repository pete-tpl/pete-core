use std::collections::HashMap;

pub type ParameterStore = HashMap<String, Parameter>;

pub struct Parameter {
    // string_value: String,
    // int_value: i128,
    // float_value: f64,
    // struct_value: ParameterStore
}

impl Parameter {
    pub fn new() -> Parameter {
        Parameter {
            // string_value: String::new(),
            // int_value: 0,
            // float_value: 0.0,
            // struct_value: HashMap::new()
        }
    }
}