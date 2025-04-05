mod any;
pub use any::Any;
use wasm_bindgen::JsValue;

pub trait Get<T> {
    fn get(&self, key: &str) -> Result<T, wasm_bindgen::JsValue>;
}

//This one is pretty sane
//It implements a getter for any type that validates
//that the returned prop is actually that type
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

//I macro'd the fuck out of this. Idk how to even explain whats going on
macro_rules! magic {
  ($(($type:ident, $type_str:literal) => $error:literal)*) => ($(

    #[doc = "Simple wrapper for a JS"]
    #[doc = $type_str]
    pub struct $type{
      pub(super) value: wasm_bindgen::JsValue,
      pub(super) context: wasm_bindgen::JsValue
    }

    //So you can get a ref to the internal JsValue
    impl std::ops::Deref for $type {
      type Target = wasm_bindgen::JsValue;

      fn deref(&self) -> &Self::Target {
        &self.value
      }
    }

    //Cast any type into Any
    impl From<$type> for Any {
      fn from(value:$type) -> Any {
        Any::new(value.value.into(), value.context)
      }
    }

    //wasm_bindgen black magic to convert the structs between JS and rust
    impl wasm_bindgen::describe::WasmDescribe for $type {
        fn describe() {
            wasm_bindgen::describe::inform(wasm_bindgen::describe::EXTERNREF);
        }
    }

    impl wasm_bindgen::convert::FromWasmAbi for $type {
        type Abi = u32;

        unsafe fn from_abi(js: Self::Abi) -> Self {
            let any = unsafe { Any::from_abi(js) };
            let value:Result<$type, JsValue> = any.try_into();
            match value {
                Ok(v) => v,
                Err(v) => {log_error(v);panic!()}
            }
        }
    }

    impl wasm_bindgen::convert::IntoWasmAbi for $type {
        type Abi = u32;

        fn into_abi(self) -> Self::Abi {
            self.value.into_abi()
        }

    }

    //Any implements future so any type can be awaited if its cast to Any first
    impl IntoFuture for $type {
      type Output = Result<Any, JsValue>;

      type IntoFuture = Any;

      fn into_future(self) -> Self::IntoFuture {
        self.into()
      }
    }

    //this should be pretty obvious what it does
    //(its the constructor)
    //...
    //(and a way to clone the internal JsValue)
    impl $type {
      pub fn new(value: wasm_bindgen::JsValue, context: wasm_bindgen::JsValue) -> Self {
          Self{value, context}
      }
      pub fn unwrap(&self) -> JsValue{
        self.value.clone()
      }
    }

    //Constantly typing $type::new(value, JsValue::undefined()) sucks
    //The only reason context exists is functions anyway
    //literally the only reason other values also have it is to make macros easier
    //also context shouldnt get lost if you do fucky casting ig
    impl From<JsValue> for $type {
       fn from(val:JsValue) -> $type {
          $type::new(val, JsValue::undefined())
       }
    }

    //I heard you like macros, so I marcod your macro
    impl_get!{
      $type, $error, String, $type_str
      $type, $error, Object, $type_str
      $type, $error, Function, $type_str
      $type, $error, Boolean, $type_str
      $type, $error, Undefined, $type_str
      $type, $error, Symbol, $type_str
      $type, $error, BigInt, $type_str
      $type, $error, Number, $type_str
    }

  )*)
}

//Again, english fucking suuuucks
magic! {
  (String, "string") => "Property {} is not a string"
  (Object, "object") => "Property {} is not an object"
  (Function, "function") => "Property {} is not a function"
  (Boolean, "bool") => "Property {} is not a boolean"
  (Undefined, "undefined") => "Property {} is not undefined"
  (Symbol, "symbol") => "Property {} is not a symbol"
  (BigInt, "bigint") => "Property {} is not a bigInt"
  (Number, "number") => "Property {} is not a number"
}

#[wasm_bindgen::prelude::wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = error)]
    pub fn log_error(s: JsValue);
}

//Probably a mem leak but idc rn. Just dont spam closures ig ¯\_(ツ)_/¯
#[macro_export]
macro_rules! js_closure {
    ($t:expr) => {
        Function::from(Box::new(|args: js_sys::Array| -> JsValue {
            let args = Any::from(Into::<JsValue>::into(args));

            let ret = $t(args);

            match ret {
                Ok(v) => v.unwrap(),
                Err(v) => {
                    bitburner_bindings::log_error(v.clone());
                    v
                }
            }
        }))
    };
}

impl Function {
    pub fn from(value: Box<dyn Fn(js_sys::Array) -> JsValue>) -> Self {
        //this is evil
        //in order to have variadic functions I need to collect all args into one array and pass that instead
        let js_wrapper =
            js_sys::Function::new_with_args("fn", "return (...args) => () => fn(args)");
        let js_wrapper = Function::new(js_wrapper.into(), JsValue::undefined());

        let closure = wasm_bindgen::closure::Closure::wrap(value);
        let closure = Any::new(closure.into_js_value().into(), JsValue::undefined());

        //This wraps the wasm closure in the argument collector wrapper
        let closure = js_wrapper
            .arg(closure)
            .call()
            .expect("If this throws black magic fuckery happened");

        Function::new(closure.unwrap(), JsValue::undefined())
    }

    //workaround for variadic function
    pub fn arg(&self, arg: Any) -> Self {
        //this binding is insane. this doesnt even work
        //but nothing broke yet sooooooooo idc
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

impl From<f64> for Number {
    fn from(v: f64) -> Self {
        Number {
            value: JsValue::from(v),
            context: JsValue::undefined(),
        }
    }
}

impl From<&str> for String {
    fn from(str: &str) -> Self {
        String {
            value: JsValue::from(str),
            context: JsValue::undefined(),
        }
    }
}

impl From<std::string::String> for String {
    fn from(v: std::string::String) -> Self {
        String {
            value: JsValue::from(v),
            context: JsValue::undefined(),
        }
    }
}

impl From<()> for Undefined {
    fn from(_: ()) -> Self {
        Undefined {
            value: JsValue::undefined(),
            context: JsValue::undefined(),
        }
    }
}
