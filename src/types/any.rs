use super::*;
use wasm_bindgen::{JsError, JsValue};

pub struct Any {
    pub(super) value: JsValue,
    pub(super) context: JsValue,
}

impl Any {
    pub fn new(value: JsValue, context: JsValue) -> Self {
        Any { value, context }
    }
}

macro_rules! try_into {
  ($($t:ident => $m:literal)*) => ($(
    impl TryInto<$t> for Any {
      type Error = JsValue;
      fn try_into(self) -> Result<$t, Self::Error> {
          if self.value.is_function() {
              return Ok($t::new(self.value.into(), self.context));
          };

          return Err(JsError::new(
              format!(
                  $m,
                  self.value.js_typeof().as_string().unwrap()
              )
              .as_str(),
          )
          .into());
      }
  }
  )*)
}

try_into! {
  Function => "Cannot cast {} into a function"
  Object => "Cannot cast {} into an object"
  Undefined => "{} is not undefined"
  BigInt =>  "Can not cast {} into a bigint"
  Boolean => "Can not cast {} into a boolean"
  Symbol => "Can not cast {} into a symbol"
  String => "Can not cast {} into a string"
}
