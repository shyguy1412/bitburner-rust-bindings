use wasm_bindgen::JsValue;

use super::any::Any;

pub struct Function {
    pub(super) value: js_sys::Function,
    pub(super) context: JsValue,
}

impl Function {
    pub fn arg(&self, arg: Any) -> Self {
        let context = if self.is_bound() {
            JsValue::undefined()
        } else {
            self.context.clone()
        };

        Self {
            value: self.value.bind1(&context, &arg.value),
            context,
        }
    }

    pub fn call(self) -> Result<Any, JsValue> {
        match self.value.call0(&self.context) {
            Ok(v) => Ok(Any::new(v, JsValue::undefined())),
            Err(v) => Err(v),
        }
    }

    fn is_bound(&self) -> bool {
        return self.value.has_own_property(&JsValue::from("prototype"));
    }
}
