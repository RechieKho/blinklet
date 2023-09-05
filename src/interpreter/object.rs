use std::{collections::HashMap, sync::Arc};
use super::{value::Value, function::{Function, NativeFunction, NativeFunctionHandler}, standard::greet};

#[derive(Clone)]
pub struct Object<'code> {
    pub content : HashMap<String, Value<'code>>
}

impl<'code> ToString for Object<'code> {
    fn to_string(&self) -> String {
        format!("<Object at {:p}>", self)
    }
}

impl<'code> Default for Object<'code> {
    fn default() -> Self {
        let mut object = Object { content: HashMap::default() };
        object.set_native_function(String::from("greet"), greet);
        object
    }

}

impl<'code> Object<'code> {
    pub fn set_native_function(&mut self, name : String, handler : NativeFunctionHandler<'code>) -> Option<Value<'code>>{
        let native_fn : Arc<dyn Function<'code> + 'code> = Arc::new(NativeFunction{ handler });
        self.content.insert(name, Value::FUNCTION(native_fn))
    }
}
