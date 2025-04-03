use proc_macro_error::emit_error;
use swc_common::SourceMap;
use swc_ecma_ast::{
    TsArrayType, TsConditionalType, TsFnOrConstructorType, TsImportType, TsIndexedAccessType,
    TsInferType, TsKeywordType, TsLitType, TsMappedType, TsOptionalType, TsParenthesizedType,
    TsRestType, TsThisType, TsTupleType, TsTypeAliasDecl, TsTypeAnn, TsTypeLit, TsTypeOperator,
    TsTypePredicate, TsTypeQuery, TsTypeRef, TsUnionOrIntersectionType,
};

use super::{error::TransformResult, safe_convert_ident};
use crate::transform::{error::Error, parse_quote};

pub fn type_alias_to_token_stream(decl: &TsTypeAliasDecl, _cm: &SourceMap) -> proc_macro::TokenStream {
    let ident: syn::Ident = safe_convert_ident(&decl.id);

    quote::quote! {
      struct #ident{}
    }
    .into()
}

pub fn type_annotation_to_type(ann: &TsTypeAnn, cm: &SourceMap) -> Option<syn::Type> {
    let result = match &*ann.type_ann {
        swc_ecma_ast::TsType::TsKeywordType(node) => keyword_type_to_type(node, cm),
        swc_ecma_ast::TsType::TsThisType(node) => this_type_to_type(node, cm),
        swc_ecma_ast::TsType::TsFnOrConstructorType(node) => fn_or_constructor_type_to_type(node, cm),
        swc_ecma_ast::TsType::TsTypeRef(node) => type_ref_to_type(node, cm),
        swc_ecma_ast::TsType::TsTypeQuery(node) => type_query_to_type(node, cm),
        swc_ecma_ast::TsType::TsTypeLit(node) => type_lit_to_type(node, cm),
        swc_ecma_ast::TsType::TsArrayType(node) => array_type_to_type(node, cm),
        swc_ecma_ast::TsType::TsTupleType(node) => tuple_type_to_type(node, cm),
        swc_ecma_ast::TsType::TsOptionalType(node) => optional_type_to_type(node, cm),
        swc_ecma_ast::TsType::TsRestType(node) => rest_type_to_type(node, cm),
        swc_ecma_ast::TsType::TsUnionOrIntersectionType(node) => {
            union_or_intersection_type_to_type(node, cm)
        }
        swc_ecma_ast::TsType::TsConditionalType(node) => conditional_type_to_type(node, cm),
        swc_ecma_ast::TsType::TsInferType(node) => infer_type_to_type(node, cm),
        swc_ecma_ast::TsType::TsParenthesizedType(node) => parenthesized_type_to_type(node, cm),
        swc_ecma_ast::TsType::TsTypeOperator(node) => type_operator_to_type(node, cm),
        swc_ecma_ast::TsType::TsIndexedAccessType(node) => indexed_access_type_to_type(node, cm),
        swc_ecma_ast::TsType::TsMappedType(node) => mapped_type_to_type(node, cm),
        swc_ecma_ast::TsType::TsLitType(node) => lit_type_to_type(node, cm),
        swc_ecma_ast::TsType::TsTypePredicate(node) => type_predicate_to_type(node, cm),
        swc_ecma_ast::TsType::TsImportType(node) => import_type_to_type(node, cm),
    };

    match result {
        Ok(ty) => Some(ty),
        Err(err) => {
            emit_error!(err);
            None
        }
    }
}

pub fn keyword_type_to_type(_ty: &TsKeywordType, _cm: &SourceMap) -> TransformResult<syn::Type> {
    parse_quote!({crate::types::Any} as syn::Type)
}

pub fn this_type_to_type(_ty: &TsThisType, _cm: &SourceMap) -> TransformResult<syn::Type> {
    parse_quote!({crate::types::Any} as syn::Type)
}

pub fn fn_or_constructor_type_to_type(_ty: &TsFnOrConstructorType, _cm: &SourceMap) -> TransformResult<syn::Type> {
    parse_quote!({crate::types::Any} as syn::Type)
}

pub fn type_ref_to_type(_ty: &TsTypeRef, _cm: &SourceMap) -> TransformResult<syn::Type> {
    parse_quote!({crate::types::Any} as syn::Type)
}

pub fn type_query_to_type(_ty: &TsTypeQuery, _cm: &SourceMap) -> TransformResult<syn::Type> {
    parse_quote!({crate::types::Any} as syn::Type)
}

pub fn type_lit_to_type(_ty: &TsTypeLit, _cm: &SourceMap) -> TransformResult<syn::Type> {
    parse_quote!({crate::types::Any} as syn::Type)
}

pub fn array_type_to_type(_ty: &TsArrayType, _cm: &SourceMap) -> TransformResult<syn::Type> {
    parse_quote!({crate::types::Any} as syn::Type)
}

pub fn tuple_type_to_type(_ty: &TsTupleType, _cm: &SourceMap) -> TransformResult<syn::Type> {
    parse_quote!({crate::types::Any} as syn::Type)
}

pub fn optional_type_to_type(_ty: &TsOptionalType, _cm: &SourceMap) -> TransformResult<syn::Type> {
    parse_quote!({crate::types::Any} as syn::Type)
}

pub fn rest_type_to_type(_ty: &TsRestType, _cm: &SourceMap) -> TransformResult<syn::Type> {
    parse_quote!({crate::types::Any} as syn::Type)
}

pub fn union_or_intersection_type_to_type(
    _ty: &TsUnionOrIntersectionType, _cm: &SourceMap,
) -> TransformResult<syn::Type> {
    parse_quote!({crate::types::Any} as syn::Type)
}

pub fn conditional_type_to_type(_ty: &TsConditionalType, _cm: &SourceMap) -> TransformResult<syn::Type> {
    parse_quote!({crate::types::Any} as syn::Type)
}

pub fn infer_type_to_type(_ty: &TsInferType, _cm: &SourceMap) -> TransformResult<syn::Type> {
    parse_quote!({crate::types::Any} as syn::Type)
}

pub fn parenthesized_type_to_type(_ty: &TsParenthesizedType, _cm: &SourceMap) -> TransformResult<syn::Type> {
    parse_quote!({crate::types::Any} as syn::Type)
}

pub fn type_operator_to_type(_ty: &TsTypeOperator, _cm: &SourceMap) -> TransformResult<syn::Type> {
    parse_quote!({crate::types::Any} as syn::Type)
}

pub fn indexed_access_type_to_type(_ty: &TsIndexedAccessType, _cm: &SourceMap) -> TransformResult<syn::Type> {
    parse_quote!({crate::types::Any} as syn::Type)
}

pub fn mapped_type_to_type(_ty: &TsMappedType, _cm: &SourceMap) -> TransformResult<syn::Type> {
    parse_quote!({crate::types::Any} as syn::Type)
}

pub fn lit_type_to_type(_ty: &TsLitType, _cm: &SourceMap) -> TransformResult<syn::Type> {
    parse_quote!({crate::types::Any} as syn::Type)
}

pub fn type_predicate_to_type(_ty: &TsTypePredicate, _cm: &SourceMap) -> TransformResult<syn::Type> {
    parse_quote!({crate::types::Any} as syn::Type)
}

pub fn import_type_to_type(ty: &TsImportType, cm: &SourceMap) -> TransformResult<syn::Type> {
    Err(Error::unsupported("Import Type", ty.span, cm))
    // parse_quote!({crate::types::Any} as syn::Type)
}
