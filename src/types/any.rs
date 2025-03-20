use bitburner_bindings_macros::into_specific;
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

into_specific!(Any);
// macro_rules! into_known {
//   ($($t:ty)*) => ($(

//     impl TryInto<Any> for $t {
//       type Error = JsValue;
//       fn try_into(self) -> $t {
//         if !self.is_function() {
//             return Err(
//                 JsError::new(&("Property '".to_owned() + key + "' is not a function")).into(),
//             );
//         };

//         Ok(Function::new(prop, self.value.clone()))
//       }
//     }

//   )*)
// }

// // String
// into_known! {
//   Object
//   Function
//   Boolean
//   Undefined
//   Symbol
//   BigInt
// }
