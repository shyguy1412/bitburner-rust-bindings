mod any;
pub use any::Any;

pub trait Get<T> {
    fn get(&self, key: &str) -> Result<T, wasm_bindgen::JsValue>;
}

macro_rules! impl_get {
  ($($type:ident, $error:literal, $parent:ident, $type_str:literal)*) => ($(
    impl Get<$type> for $parent {
      fn get(&self, key: &str) -> Result<$type, wasm_bindgen::JsValue> {
          let prop = js_sys::Reflect::get(&self.value, &wasm_bindgen::JsValue::from(key)).unwrap();

          if prop.js_typeof() != $type_str {
              return Err(
                  wasm_bindgen::JsError::new(format!($error, key).as_str()).into(),
              );
          };

          Ok($type::new(prop.into(), self.value.clone().into()))
      }
    }
  )*)
}

macro_rules! magic {
  ($(($type:ident, $type_str:literal) => $error:literal)*) => ($(

    pub struct $type{
      pub(super) value: wasm_bindgen::JsValue,
      pub(super) context: wasm_bindgen::JsValue
    }

    impl std::ops::Deref for $type {
      type Target = wasm_bindgen::JsValue;

      fn deref(&self) -> &Self::Target {
        &self.value
      }
    }

    impl Into<Any> for $type {
      fn into(self) -> Any {
        Any::new(self.value.into(), self.context)
      }
    }

    impl $type {
      pub fn new(val: wasm_bindgen::JsValue, context: wasm_bindgen::JsValue) -> Self {
          Self{
            value: val.into(),
            context
          }
      }
    }

    impl_get!{
      $type, $error, String, $type_str
      $type, $error, Object, $type_str
      $type, $error, Function, $type_str
      $type, $error, Boolean, $type_str
      $type, $error, Undefined, $type_str
      $type, $error, Symbol, $type_str
      $type, $error, BigInt, $type_str
    }

  )*)
}

magic! {
  (String, "string") => "Property {} is not a string"
  (Object, "object") => "Property {} is not an object"
  (Function, "function") => "Property {} is not a function"
  (Boolean, "bool") => "Property {} is not a boolean"
  (Undefined, "undefined") => "Property {} is not undefined"
  (Symbol, "symbol") => "Property {} is not a symbol"
  (BigInt, "bigint") => "Property {} is not a bigInt"
}

impl Function {
    pub fn arg(&self, arg: Any) -> Self {
        let context = if self.is_bound() {
            wasm_bindgen::JsValue::undefined()
        } else {
            self.context.clone()
        };

        let this: js_sys::Function = self.value.clone().into();

        Self {
            value: this.bind1(&context, &arg.value).into(),
            context,
        }
    }

    pub fn call(self) -> Result<Any, wasm_bindgen::JsValue> {
        let this: js_sys::Function = self.value.clone().into();

        match this.call0(&self.context) {
            Ok(v) => Ok(Any::new(v, wasm_bindgen::JsValue::undefined())),
            Err(v) => Err(v),
        }
    }

    fn is_bound(&self) -> bool {
        let this: js_sys::Function = self.value.clone().into();

        return !this.has_own_property(&wasm_bindgen::JsValue::from("prototype"));
    }
}
