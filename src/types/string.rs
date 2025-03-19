use wasm_bindgen::{JsError, JsValue};

use super::{Function, Get};

pub struct String(pub(super) JsValue, pub(super) JsValue);

impl String {
    pub fn new(val: JsValue) -> Self {
        Self(val, JsValue::undefined())
    }
}

impl Get<Function> for String {
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

impl Get<String> for String {
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