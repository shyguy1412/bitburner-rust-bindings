use super::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsError, JsValue};
use std::cell::RefCell;
use std::rc::Rc;

pub struct Any {
    pub(super) value: JsValue,
    pub(super) context: JsValue,
    inner: Option<Rc<RefCell<Inner>>>,
}

struct Inner {
    result: Option<Result<Any, JsValue>>,
    task: Option<std::task::Waker>,
    callbacks: Option<(Closure<dyn FnMut(JsValue)>, Closure<dyn FnMut(JsValue)>)>,
}

impl Any {
    pub fn new(value: JsValue, context: JsValue) -> Self {
        Any {
            value,
            context,
            inner: None,
        }
    }
    pub fn unwrap(&self) -> JsValue {
        self.value.clone()
    }
}

impl From<JsValue> for Any {
    fn from(value: JsValue) -> Self {
        Any::new(value, JsValue::undefined())
    }
}

impl From<()> for Any {
    fn from(_: ()) -> Self {
        Any::new(JsValue::undefined(), JsValue::undefined())
    }
}


impl std::ops::Deref for Any {
    type Target = wasm_bindgen::JsValue;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

macro_rules! try_into {
  ($(($t:ident, $ts:literal) => $m:literal)*) => ($(
    impl TryInto<$t> for Any {
        type Error = JsValue;

        fn try_into(self) -> Result<$t, Self::Error> {

            if self.js_typeof() != $ts {
                return Err(JsError::new(
                    format!(
                        $m,
                        self.value.js_typeof().as_string().unwrap()
                    )
                    .as_str(),
                ).into());
            };

            return Ok($t::new(self.value.into(), self.context));
        }
    }
  )*)
}

try_into! {
  (Function, "function") => "Cannot cast {} into a function"
  (Object, "object") => "Cannot cast {} into an object"
  (Undefined, "undefined") => "{} is not undefined"
  (BigInt, "bigInt") =>  "Can not cast {} into a bigint"
  (Boolean, "boolean") => "Can not cast {} into a boolean"
  (Symbol, "symbol") => "Can not cast {} into a symbol"
  (String, "string") => "Can not cast {} into a string"
}

impl Future for Any {
    type Output = Result<Any, JsValue>;

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context,
    ) -> std::task::Poll<Self::Output> {
        if self.inner.is_none() {
            //Stolen from JsFuture
            let state = Rc::new(RefCell::new(Inner {
                result: None,
                task: None,
                callbacks: None,
            }));

            fn finish(state: &RefCell<Inner>, val: Result<Any, JsValue>) {
                let task = {
                    let mut state = state.borrow_mut();
                    debug_assert!(state.callbacks.is_some());
                    debug_assert!(state.result.is_none());

                    // First up drop our closures as they'll never be invoked again and
                    // this is our chance to clean up their state.
                    drop(state.callbacks.take());

                    // Next, store the value into the internal state.
                    state.result = Some(val);
                    state.task.take()
                };

                // And then finally if any task was waiting on the value wake it up and
                // let them know it's there.
                if let Some(task) = task {
                    task.wake()
                }
            }

            let resolve = {
                let state = state.clone();
                Closure::once(move |val| finish(&state, Ok(Any::new(val, JsValue::undefined()))))
            };

            let reject = {
                let state = state.clone();
                Closure::once(move |val| finish(&state, Err(val)))
            };

            let _ = js_sys::Promise::resolve(&self.value).then2(&resolve, &reject);

            state.borrow_mut().callbacks = Some((resolve, reject));
            self.inner = Some(state);
        }

        match &self.inner {
            Some(inner) => {
                // If our value has come in then we return it...
                if let Some(val) = inner.borrow_mut().result.take() {
                    return std::task::Poll::Ready(val);
                }

                // ... otherwise we arrange ourselves to get woken up once the value
                // does come in
                inner.borrow_mut().task = Some(cx.waker().clone());
                std::task::Poll::Pending
            }
            None => panic!("wtf?"),
        }
    }
}
