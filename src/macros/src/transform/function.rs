use swc_ecma_ast::{ArrayPat, BindingIdent, ObjectPat, RestPat, TsFnParam, TsMethodSignature};

use crate::transform::{parse_quote, safe_convert_ident, r#type::type_annotation_to_type};

pub fn ts_fn_param_to_arg(param: &TsFnParam) -> syn::FnArg {
    match param {
        TsFnParam::Ident(node) => binding_ident_to_arg(node),
        TsFnParam::Array(node) => array_arg_to_arg(node),
        TsFnParam::Rest(node) => rest_arg_to_arg(node),
        TsFnParam::Object(node) => object_arg_to_arg(node),
    }
}

pub fn array_arg_to_arg(_: &ArrayPat) -> syn::FnArg {
    todo!("Array destructuring")
}
pub fn rest_arg_to_arg(arg: &RestPat) -> syn::FnArg {
    match arg.arg.as_ref() {
        swc_ecma_ast::Pat::Ident(node) => binding_ident_to_arg(node),
        _ => panic!("A spread param should just be a fucking ident??"),
    }
}
pub fn object_arg_to_arg(_: &ObjectPat) -> syn::FnArg {
    todo!("Object destructuring")
}

pub fn binding_ident_to_arg(binding_ident: &BindingIdent) -> syn::FnArg {
    let arg_type = binding_ident
        .type_ann
        .as_ref()
        .and_then(|type_ann| type_annotation_to_type(&type_ann))
        .unwrap_or(parse_quote!({crate::types::Any} as syn::Type  => ""));

    let ident = safe_convert_ident(&binding_ident.id).unwrap();
    parse_quote!({#ident:#arg_type} as syn::FnArg => "Not an arg")
}

pub fn method_signature_to_impl_item_fn(method: &TsMethodSignature) -> syn::ImplItemFn {
    let ident = method
        .key
        .as_ident()
        .and_then(safe_convert_ident)
        .expect("computed method signatures are not supported yet");

    let args: Vec<_> = method.params.iter().map(ts_fn_param_to_arg).collect();

    parse_quote!({
        pub fn #ident(&self, #(#args),*){} 
    } as syn::ImplItemFn => "")
}
