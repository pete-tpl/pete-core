use std::collections::HashMap;

pub type VariableStore = HashMap<String, Variable>;

enum VariableType {
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

pub struct Variable {
    value: Value,
    string_value: String,
    struct_value: VariableStore,
    param_type: VariableType
}

impl Variable {
    pub fn new() -> Variable {
        Variable {
            value: Value { int_value: 0 },
            string_value: String::new(),
            param_type: VariableType::StringType,
            struct_value: HashMap::new(),
        }
    }

    pub fn new_from_boolean(value: bool) -> Variable {
        let mut p = Variable::new();
        p.set_boolean_value(value);
        p
    }

    pub fn new_from_string(string: String) -> Variable {
        let mut p = Variable::new();
        p.set_string_value(string);
        p
    }

    pub fn new_from_str(string: &str) -> Variable {
        let mut p = Variable::new();
        p.set_string_value(String::from(string));
        p
    }

    pub fn new_from_int(value: i128) -> Variable {
        let mut p = Variable::new();
        p.set_int_value(value);
        p
    }

    pub fn new_from_float(value: f64) -> Variable {
        let mut p = Variable::new();
        p.set_float_value(value);
        p
    }

    pub fn set_boolean_value(&mut self, value: bool) {
        self.param_type = VariableType::Boolean;
        self.value = Value { boolean_value: value };
    }

    pub fn set_float_value(&mut self, value: f64) {
        self.param_type = VariableType::Float;
        self.value = Value { float_value: value };
    }

    pub fn set_int_value(&mut self, value: i128) {
        self.param_type = VariableType::Int;
        self.value = Value { int_value: value };
    }

    pub fn set_string_value(&mut self, value: String) {
        self.param_type = VariableType::StringType;
        self.string_value = value;
    }

    pub fn set_struct_value(&mut self, value: VariableStore) {
        self.param_type = VariableType::Struct;
        self.struct_value = value;
    }

    pub fn get_string_value(&self) -> String {
        unsafe {
            match self.param_type {
                VariableType::Boolean => String::from(if self.value.boolean_value == true { "true" } else { "false" }),
                VariableType::Float => self.value.float_value.to_string(),
                VariableType::Int => self.value.int_value.to_string(),
                VariableType::StringType => self.string_value.clone(),
                VariableType::Struct => panic!("Not implemented") // TODO: implement
            }
        }
    }

    pub fn get_boolean_value(&self) -> bool {
        unsafe {
            match self.param_type {
                VariableType::Boolean => self.value.boolean_value,
                VariableType::Float => self.value.float_value != 0.0,
                VariableType::Int => self.value.int_value != 0,
                VariableType::StringType => self.string_value != "",
                VariableType::Struct => panic!("Not implemented") // TODO: implement
            }
        }
    }

    pub fn get_int_value(&self) -> Option<i128> {
        unsafe {
            match self.param_type {
                VariableType::Int => Some(self.value.int_value),
                _ => None,
            }
        }
    }

    pub fn get_float_value(&self) -> Option<f64> {
        unsafe {
            match self.param_type {
                VariableType::Float => Some(self.value.float_value),
                VariableType::Int => Some(self.value.int_value as f64),
                _ => None,
            }
        }
    }

    pub fn clone(&self) -> Variable {
        let mut dest = Variable::new();
        unsafe {
            match self.param_type {
                VariableType::Boolean => { dest.set_boolean_value(self.value.boolean_value); },
                VariableType::Float => { dest.set_float_value(self.value.float_value); },
                VariableType::Int => { dest.set_int_value(self.value.int_value); },
                VariableType::StringType => { dest.set_string_value(self.string_value.clone()); },
                VariableType::Struct => { dest.set_struct_value(clone_parameter_store(&self.struct_value)); }
            }
    
        }
        dest
    }

    pub fn as_string(&self) -> String {
        self.get_string_value()
    }
}

fn clone_parameter_store(src: &VariableStore) -> VariableStore {
    let mut dest = VariableStore::new();
    for (k, v) in src.iter() {
        dest.insert(k.to_string(), v.clone());
    }
    dest
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_common_variable_get_boolean_value_bool() {
        let value = Variable::new_from_boolean(true);
        assert_eq!(value.get_boolean_value(), true);
        let value = Variable::new_from_boolean(false);
        assert_eq!(value.get_boolean_value(), false);
    }

    #[test]
    fn test_common_variable_get_boolean_value_string() {
        let value = Variable::new_from_str("Hello");
        assert_eq!(value.get_boolean_value(), true);
        let value = Variable::new_from_str("");
        assert_eq!(value.get_boolean_value(), false);
    }

    #[test]
    fn test_common_variable_get_boolean_value_int() {
        let value = Variable::new_from_int(999);
        assert_eq!(value.get_boolean_value(), true);
        let value = Variable::new_from_int(0);
        assert_eq!(value.get_boolean_value(), false);
    }
}