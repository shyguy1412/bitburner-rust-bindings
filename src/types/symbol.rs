use wasm_bindgen::JsValue;

pub struct Symbol{
  pub(super) value: JsValue,
  pub(super) context: JsValue
}
