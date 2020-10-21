use std::collections::HashMap;

pub type ParameterStore = HashMap<String, Parameter>;

enum ParameterType {
    Boolean,
    Float,
    Int,
    StringType,
    Struct
}

union Value {
    boolean_value: bool,
    float_value: f64,
    int_value: i128,
}

#[allow(dead_code)]
pub struct Parameter {
    value: Value,
    string_value: String,
    struct_value: ParameterStore,
    param_type: ParameterType
}

impl Parameter {
    pub fn new() -> Parameter {
        Parameter {
            value: Value { int_value: 0 },
            string_value: String::new(),
            param_type: ParameterType::StringType,
            struct_value: HashMap::new(),
        }
    }

    pub fn new_from_string(string: String) -> Parameter {
        let mut p = Parameter::new();
        p.set_string_value(string);
        p
    }

    pub fn new_from_str(string: &str) -> Parameter {
        let mut p = Parameter::new();
        p.set_string_value(String::from(string));
        p
    }

    pub fn new_from_int(value: i128) -> Parameter {
        let mut p = Parameter::new();
        p.set_int_value(value);
        p
    }

    pub fn new_from_float(value: f64) -> Parameter {
        let mut p = Parameter::new();
        p.set_float_value(value);
        p
    }

    pub fn set_boolean_value(&mut self, value: bool) {
        self.param_type = ParameterType::Boolean;
        self.value = Value { boolean_value: value };
    }

    pub fn set_float_value(&mut self, value: f64) {
        self.param_type = ParameterType::Float;
        self.value = Value { float_value: value };
    }

    pub fn set_int_value(&mut self, value: i128) {
        self.param_type = ParameterType::Int;
        self.value = Value { int_value: value };
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
        unsafe {
            match self.param_type {
                ParameterType::Boolean => String::from(if self.value.boolean_value == true { "true" } else { "false" }),
                ParameterType::Float => self.value.float_value.to_string(),
                ParameterType::Int => self.value.int_value.to_string(),
                ParameterType::StringType => self.string_value.clone(),
                ParameterType::Struct => panic!("Not implemented") // TODO: implement
            }
        }
    }

    pub fn get_int_value(&self) -> Option<i128> {
        unsafe {
            match self.param_type {
                ParameterType::Int => Some(self.value.int_value),
                _ => None,
            }
        }
    }

    pub fn get_float_value(&self) -> Option<f64> {
        unsafe {
            match self.param_type {
                ParameterType::Float => Some(self.value.float_value),
                ParameterType::Int => Some(self.value.int_value as f64),
                _ => None,
            }
        }
    }

    pub fn clone(&self) -> Parameter {
        let mut dest = Parameter::new();
        unsafe {
            match self.param_type {
                ParameterType::Boolean => { dest.set_boolean_value(self.value.boolean_value); },
                ParameterType::Float => { dest.set_float_value(self.value.float_value); },
                ParameterType::Int => { dest.set_int_value(self.value.int_value); },
                ParameterType::StringType => { dest.set_string_value(self.string_value.clone()); },
                ParameterType::Struct => { dest.set_struct_value(clone_parameter_store(&self.struct_value)); }
            }
    
        }
        dest
    }

    pub fn as_string(&self) -> String {
        self.get_string_value()
    }
}

fn clone_parameter_store(src: &ParameterStore) -> ParameterStore {
    let mut dest = ParameterStore::new();
    for (k, v) in src.iter() {
        dest.insert(k.to_string(), v.clone());
    }
    dest
}