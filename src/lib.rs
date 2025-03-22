use wasm_bindgen::JsValue;

mod types;
pub use types::*;

pub use bitburner_bindings_macros::bb_bindgen;

pub use uuid;

pub struct NS {
    _ns: Object,
    pub args: Object,
}

impl NS {

    pub fn tprint(&self, message: String) -> Result<Undefined, JsValue> {
        let tprint: Function = self._ns.get("tprint")?;

        tprint.arg(message.into()).call()?;
        Ok(().into())
    }

    pub fn sleep(&self, amount: Number) -> Result<Object, JsValue> {
        let sleep: Function = self._ns.get("sleep")?;
        let retval: Object = sleep.arg(amount.into()).call()?.try_into()?;

        Ok(retval)
    }

    #[allow(non_snake_case)]
    pub fn atExit(&self, callback: Function, id:String) -> Result<Undefined, JsValue> {
        let at_exit: Function = self._ns.get("atExit")?;

        at_exit.arg(callback.into()).arg(id.into()).call()?;

        Ok(().into())
    }
}

impl TryFrom<JsValue> for NS {
    type Error = JsValue;

    fn try_from(value: JsValue) -> Result<Self, Self::Error> {
        let _ns = Object::from(value);
        Ok(NS {
            args: _ns.get(&"args")?,
            _ns,
        })
    }
}

pub fn v4uuid() -> String{
    let my_uuid = uuid::Uuid::new_v4().as_hyphenated().to_string().as_str().to_owned();
    my_uuid.into()
}
