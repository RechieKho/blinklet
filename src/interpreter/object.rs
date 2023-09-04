use std::sync::Arc;
use super::{value::{ValueMap, Value}, function::NativeFunction, standard::greet};

#[derive(Clone)]
pub struct Object {
    pub name : String,
    pub values : ValueMap,
    pub prototypes : Vec<Arc<Object>>,
}

impl ToString for Object {
    fn to_string(&self) -> String {
        format!("<Object at {:p}>", self)
    }
}

impl Default for Object {
    fn default() -> Self {
        let mut object = Object { name: String::from("Object"), values: ValueMap::default(), prototypes: Vec::new() };
        
        {
            let print_fn_name = String::from("greet");
            let native_fn = NativeFunction { name: print_fn_name.clone(), handler: greet };
            object.values.insert(print_fn_name, Value::FUNCTION(Arc::new(native_fn)));
        }

        object
    }
}

impl Object {
    pub fn freeze(&self) -> Arc<Object> {
        Arc::new(self.clone())
    }

    pub fn get(&self, k : &String) -> Option<&Value> {
        let optional_value = self.values.get(k);
        if optional_value.is_some() { return optional_value; }
        for prototype in self.prototypes.iter().rev() {
            let optional_value = prototype.values.get(k);
            if optional_value.is_some() { return optional_value; }
        }
        None
    }
}
