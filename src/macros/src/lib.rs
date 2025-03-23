use proc_macro::{Span, TokenStream};
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn bb_bindgen(_: TokenStream, input: TokenStream) -> TokenStream {
    let mut body = parse_macro_input!(input as syn::ItemFn);

    let og_ident = body.sig.ident.clone();
    let fn_ident = &("__bb_bind_".to_owned() + &body.sig.ident.to_string());
    body.sig.ident = syn::Ident::new(fn_ident, Span::call_site().into());
    let fn_ident = body.sig.ident.clone();

    quote::quote! {
        #[wasm_bindgen::prelude::wasm_bindgen]
        pub async fn #og_ident(val:JsValue) -> Result<(), wasm_bindgen::JsValue>{
            let ns = Box::leak(Box::new(bitburner_bindings::NS::try_from(val.clone())?));

            let ret = #fn_ident(ns).await;
        
            ns.atExit(bitburner_bindings::js_closure!(|_: Any| -> Result<Any, JsValue> {
                let ns = unsafe {Box::from_raw(ns as *const NS as *mut NS)};
                drop(ns);
                Ok(bitburner_bindings::Any::from(
                    wasm_bindgen::JsValue::undefined(),
                ))
            }),bitburner_bindings::v4uuid())?;
        
            ret
        }

        #body
    }
    .into()
}
