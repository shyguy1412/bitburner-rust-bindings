use wasm_bindgen::{JsError, JsValue};

use super::{Function, Get, String};

#[derive(Clone)]
pub struct Object(JsValue);


impl Object {
    pub fn new(val: JsValue) -> Self {
        Self(val)
    }
}


impl Get<Function> for Object {
    fn get(&self, key: &str) -> Result<Function, JsValue> {
        let prop = js_sys::Reflect::get(&self.0, &JsValue::from(key)).unwrap();

        if !prop.is_function() {
            return Err(
                JsError::new(&("Property '".to_owned() + key + "' is not a function")).into(),
            );
        };

        Ok(Function::new(prop.into(), self.0.clone()))
    }
}

impl Get<Object> for Object {
    fn get(&self, key: &str) -> Result<Object, JsValue> {
        let prop = js_sys::Reflect::get(&self.0, &JsValue::from(key)).unwrap();

        if !prop.is_object() {
            return Err(
                JsError::new(&("Property '".to_owned() + key + "' is not an object")).into(),
            );
        };

        Ok(Object::new(prop.into()))
    }
}

impl Get<String> for Object {
    fn get(&self, key: &str) -> Result<String, JsValue> {
        let prop = js_sys::Reflect::get(&self.0, &JsValue::from(key)).unwrap();

        if !prop.is_string() {
            return Err(
                JsError::new(&("Property '".to_owned() + key + "' is not a string")).into(),
            );
        };

        Ok(String::new(prop.into()))
    }
}

