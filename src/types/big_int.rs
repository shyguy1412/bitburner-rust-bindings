use wasm_bindgen::JsValue;

pub struct BigInt{
  pub(super) value: JsValue,
  pub(super) context: JsValue
}
