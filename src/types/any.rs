use super::*;
use wasm_bindgen::{JsError, JsValue};
use std::boxed::Box;
use std::rc::Rc;
use core::cell::RefCell;
use core::fmt;
use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll, Waker};
use js_sys::Promise;
use wasm_bindgen::prelude::*;


pub struct Any {
    pub(super) value: JsValue,
    pub(super) context: JsValue,
    inner:Option<Inner>
}

impl Any {
    pub fn new(value: JsValue, context: JsValue) -> Self {
        Any { value, context, inner: None }
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

struct Inner {
    result: Option<Result<JsValue, JsValue>>,
    task: Option<Waker>,
    callbacks: Option<(Closure<dyn FnMut(JsValue)>, Closure<dyn FnMut(JsValue)>)>,
}

pub struct JsFuture {
    inner: Rc<RefCell<Inner>>,
}

impl Future for Any {
    type Output = Result<Any, JsValue>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let mut inner = self.inner.borrow_mut();

        // If our value has come in then we return it...
        if let Some(val) = inner.result.take() {
            return Poll::Ready(val);
        }

        // ... otherwise we arrange ourselves to get woken up once the value
        // does come in
        inner.task = Some(cx.waker().clone());
        Poll::Pending
    }
}

// impl IntoFuture for Any{
//     type Output;

//     type IntoFuture;

//     fn into_future(self) -> Self::IntoFuture {
//         todo!()
//     }
// }

// impl Future for Any {
//     type Output = Result<Any, JsValue>;

//     fn poll(
//         self: std::pin::Pin<&mut Self>,
//         cx: &mut std::task::Context<'_>,
//     ) -> std::task::Poll<Self::Output> {
//         let promise = js_sys::Promise::resolve(&self.value);

//         let state = Rc::new(RefCell::new(Inner {
//             result: None,
//             task: None,
//             callbacks: None,
//         }));

//         log("uwu");


//         fn finish(state: &RefCell<Inner>, val: Result<JsValue, JsValue>) {
//             let task = {
//                 let mut state = state.borrow_mut();
//                 debug_assert!(state.callbacks.is_some());
//                 debug_assert!(state.result.is_none());

//                 // First up drop our closures as they'll never be invoked again and
//                 // this is our chance to clean up their state.
//                 drop(state.callbacks.take());

//                 // Next, store the value into the internal state.
//                 state.result = Some(val);
//                 state.task.take()
//             };

//             // And then finally if any task was waiting on the value wake it up and
//             // let them know it's there.
//             if let Some(task) = task {
//                 task.wake()
//             }
//         }

//         let resolve = {
//             let state = state.clone();
//             Closure::once(move |val| finish(&state, Ok(val)))
//         };

//         let reject = {
//             let state = state.clone();
//             Closure::once(move |val| finish(&state, Err(val)))
//         };

//         let _ = promise.then2(&resolve, &reject);

//         std::task::Poll::Pending

//     }
// }

// impl Future for Any {
//     type Output = Result<Any, JsValue>;

//     fn poll(
//         self: std::pin::Pin<&mut Self>,
//         cx: &mut std::task::Context<'_>,
//     ) -> std::task::Poll<Self::Output> {
//         let promise = js_sys::Promise::resolve(&self.value);
//         let mut result = wasm_bindgen_futures::JsFuture::from(promise);

//         log("uwu");

//         match JsFuture::poll(std::pin::Pin::new(&mut result), cx) {
//             std::task::Poll::Ready(v) => match v {
//                 Ok(v) => std::task::Poll::Ready(Ok(Any::new(v, JsValue::undefined()))),
//                 Err(v) => std::task::Poll::Ready(Err(v)),
//             },
//             std::task::Poll::Pending => std::task::Poll::Pending,
//         }
//     }
// }

#[wasm_bindgen::prelude::wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen::prelude::wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

