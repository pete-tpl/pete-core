use std::collections::HashMap;

pub type ParameterStore = HashMap<String, Parameter>;

enum ParameterType {
    Boolean,
    Float,
    Int,
    StringType,
    Struct
}

#[allow(dead_code)]
pub struct Parameter {
    boolean_value: bool,
    float_value: f64,
    int_value: i128,
    string_value: String,
    struct_value: ParameterStore,
    param_type: ParameterType
}

impl Parameter {
    pub fn new() -> Parameter {
        Parameter {
            boolean_value: false,
            string_value: String::new(),
            int_value: 0,
            float_value: 0.0,
            param_type: ParameterType::StringType,
            struct_value: HashMap::new(),
        }
    }

    pub fn set_boolean_value(&mut self, value: bool) {
        self.param_type = ParameterType::Boolean;
        self.boolean_value = value;
    }

    pub fn set_float_value(&mut self, value: f64) {
        self.param_type = ParameterType::Float;
        self.float_value = value;
    }

    pub fn set_int_value(&mut self, value: i128) {
        self.param_type = ParameterType::Int;
        self.int_value = value;
    }

    pub fn set_string_value(&mut self, value: String) {
        self.param_type = ParameterType::StringType;
        self.string_value = value;
    }

    pub fn set_struct_value(&mut self, value: ParameterStore) {
        self.param_type = ParameterType::Struct;
        self.struct_value = value;
    }

    pub fn get_string_value(&self) -> String {
        match self.param_type {
            ParameterType::Boolean => String::from(if self.boolean_value == true { "true" } else { "false" }),
            ParameterType::Float => self.float_value.to_string(),
            ParameterType::Int => self.int_value.to_string(),
            ParameterType::StringType => self.string_value.clone(),
            ParameterType::Struct => String::new() // TODO: implement
        }
    }
}