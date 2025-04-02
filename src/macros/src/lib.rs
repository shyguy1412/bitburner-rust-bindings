use proc_macro::{Span, TokenStream};
use swc_ecma_ast::Decl;
use syn::parse_macro_input;

mod ast;
use ast::get_ast_for_dts;
mod transform;
use transform::declaration_to_struct_token_stream;

//This wraps the main function with some casting and memory management
#[proc_macro_attribute]
pub fn bb_bindgen(_: TokenStream, input: TokenStream) -> TokenStream {
    let mut body = parse_macro_input!(input as syn::ItemFn);

    let og_ident = body.sig.ident.clone();
    let fn_ident = format!("__bb_bind_{}", body.sig.ident);
    body.sig.ident = syn::Ident::new(fn_ident.as_str(), Span::call_site().into());
    let fn_ident = body.sig.ident.clone();

    quote::quote! {
        #[wasm_bindgen::prelude::wasm_bindgen]
        pub async fn #og_ident(val:JsValue) -> Result<(), wasm_bindgen::JsValue>{
            let ns = Box::leak(Box::new(bitburner_bindings::NS::try_from(val.clone())?));

            std::panic::set_hook(Box::new(|err| {
                bitburner_bindings::log_error(err.to_string().into())
            }));

            let ret = #fn_ident(ns).await;

            drop(std::panic::take_hook());

            //This is unsafe af. Its so easy to write code that uses NS after cleanup
            //idc tho, better cause errors than memory leaks
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

#[proc_macro]
pub fn from_dts(input: TokenStream) -> TokenStream {
    let path = parse_macro_input!(input as syn::LitStr).value();

    let contents = std::fs::read_to_string(path).expect("Dont be silly. Give me an actual path");

    let module = get_ast_for_dts(&contents).expect("fucky typescript??");

    let structs_stream: TokenStream = module
        .body
        .iter()
        .filter_map(|item| match item.is_stmt() {
            true => item.as_stmt().and_then(|item| item.as_decl()),
            false => item
                .as_module_decl()
                .and_then(|item| item.as_export_decl())
                .and_then(|item| Some(&item.decl)),
        })
        .filter_map(|decl| match decl {
            Decl::Using(_) => None,
            Decl::TsModule(_) => None,
            node => Some(declaration_to_struct_token_stream(&node)),
        })
        .fold(TokenStream::new(), |mut prev, cur| {
            prev.extend(cur);
            prev
        });

    structs_stream
}
