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

    impl IntoFuture for $type {
      type Output = Result<Any, JsValue>;

      type IntoFuture = Any;

      fn into_future(self) -> Self::IntoFuture {
        self.into()
      }
    }

    impl $type {
      pub fn new(value: wasm_bindgen::JsValue, context: wasm_bindgen::JsValue) -> Self {
          Self{value, context}
      }
      pub fn unwrap(&self) -> JsValue{
        self.value.clone()
      }
    }

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

impl Into<Number> for f64 {
    fn into(self) -> Number {
        Number {
            value: JsValue::from(self),
            context: JsValue::undefined(),
        }
    }
}

impl Into<String> for &str {
    fn into(self) -> String {
        String {
            value: JsValue::from(self),
            context: JsValue::undefined(),
        }
    }
}

impl Into<String> for std::string::String {
    fn into(self) -> String {
        String {
            value: JsValue::from(self),
            context: JsValue::undefined(),
        }
    }
}

impl Into<Undefined> for () {
    fn into(self) -> Undefined {
        Undefined {
            value: JsValue::undefined(),
            context: JsValue::undefined(),
        }
    }
}
