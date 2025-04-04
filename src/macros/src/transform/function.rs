use itertools::Itertools;
use proc_macro_error::emit_error;
use swc_common::SourceMap;
use swc_ecma_ast::{ArrayPat, BindingIdent, ObjectPat, RestPat, TsFnParam, TsMethodSignature};
use syn::FnArg;

use crate::transform::{parse_quote, safe_convert_ident, r#type::type_annotation_to_type};

use super::error::{Error, TransformResult};

pub fn ts_fn_param_to_arg(param: &TsFnParam, cm: &SourceMap) -> TransformResult<syn::FnArg> {
    match param {
        TsFnParam::Ident(node) => binding_ident_to_arg(node, cm),
        TsFnParam::Array(node) => array_arg_to_arg(node, cm),
        TsFnParam::Rest(node) => rest_arg_to_arg(node, cm),
        TsFnParam::Object(node) => object_arg_to_arg(node, cm),
    }
}

pub fn array_arg_to_arg(arg: &ArrayPat, cm: &SourceMap) -> TransformResult<syn::FnArg> {
    Err(Error::unsupported("array argument", arg.span, cm))
}
pub fn rest_arg_to_arg(arg: &RestPat, cm: &SourceMap) -> TransformResult<syn::FnArg> {
    match arg.arg.as_ref() {
        swc_ecma_ast::Pat::Ident(node) => binding_ident_to_arg(node, &cm),
        _ => Err(Error::fuck_you("This shouldnt even be valid syntax?", arg.span, cm)),
    }
}
pub fn object_arg_to_arg(arg: &ObjectPat, cm: &SourceMap) -> TransformResult<syn::FnArg> {
    Err(Error::unsupported("object argument", arg.span, cm))
}

pub fn binding_ident_to_arg(binding_ident: &BindingIdent, cm: &SourceMap) -> TransformResult<syn::FnArg> {
    let arg_type = binding_ident
        .type_ann
        .as_ref()
        .and_then(|type_ann| type_annotation_to_type(&type_ann, cm))
        .unwrap_or(parse_quote!({crate::types::Any} as syn::Type)?);

    let ident = safe_convert_ident(&binding_ident.id, cm);
    parse_quote!({#ident:#arg_type} as syn::FnArg)
}

pub fn method_signature_to_impl_item_fn(method: &TsMethodSignature, cm: &SourceMap) -> TransformResult<syn::ImplItemFn> {
    let ident = method.key.as_ident().map(|ident|safe_convert_ident(ident, cm));

    let (args, errors): (Vec<FnArg>, Vec<Error>) = method
        .params
        .iter()
        .map(|param|ts_fn_param_to_arg(param, cm))
        .partition_result();

    if errors.len() != 0 {
        errors.into_iter().for_each(|e| emit_error!(e));
    }

    parse_quote!({
        pub fn #ident(&self, #(#args),*){} 
    } as syn::ImplItemFn)
}
