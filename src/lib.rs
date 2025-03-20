use wasm_bindgen::JsValue;
use wasm_bindgen::describe::EXTERNREF;
use wasm_bindgen::describe::WasmDescribe;
use wasm_bindgen::describe::inform;

mod types;
pub use types::*;

pub use bitburner_bindings_macros::bb_bindgen;

pub struct NS {
    _ns: Object,
    pub args: Object,
}

impl WasmDescribe for NS {
    fn describe() {
        inform(EXTERNREF)
    }
}

impl NS {
    pub fn tprint(&self, message: String) -> Result<(), JsValue> {
        let tprint: Function = self._ns.get("tprint")?;

        tprint.arg(message.into()).call()?;
        Ok(())
    }

    pub fn sleep(&self, amount: Number) -> Result<Object, JsValue>{
        let sleep: Function = self._ns.get("sleep")?;
        
        let retval:Object = sleep.arg(amount.into()).call()?.try_into()?;

        Ok(retval)
    }
}

impl TryFrom<JsValue> for NS {
    type Error = JsValue;

    fn try_from(value: JsValue) -> Result<Self, Self::Error> {
        let _ns = Object::new(value, JsValue::undefined());
        Ok(NS {
            args: _ns.get(&"args")?,
            _ns,
        })
    }
}
