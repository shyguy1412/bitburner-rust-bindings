mod object;
pub use object::Object;

mod string;
pub use string::String;

mod function;
pub use function::Function;

mod boolean;
pub use boolean::Boolean;

mod undefined;
pub use undefined::Undefined;

mod symbol;
pub use symbol::Symbol;

mod big_int;
pub use big_int::BigInt;

mod any;
pub use any::Any;

pub trait Get<T> {
    fn get(&self, key: &str) -> Result<T, wasm_bindgen::JsValue>;
}

macro_rules! magic {
  ($($t:ident)*) => ($(
    impl std::ops::Deref for $t {
      type Target = wasm_bindgen::JsValue;
      
      fn deref(&self) -> &Self::Target {
        &self.0
      }
    }
    
    impl Into<Any> for $t {
      fn into(self) -> Any {
        Any(self.0.into(), self.1)
      }
    }
  )*)
}

magic! {
  String
  Object
  Function
  Boolean
  Undefined
  Symbol
  BigInt
}
