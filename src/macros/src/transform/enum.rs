use proc_macro_error::{abort, emit_call_site_warning};
use swc_common::SourceMap;
use swc_ecma_ast::{TsEnumDecl, TsEnumMember};

use super::{parse_quote, parse_string, safe_convert_ident};

fn enum_member_to_match_arm(
    ident: &syn::Ident,
    member: &TsEnumMember,
    _cm: &SourceMap,
) -> (syn::Variant, Option<syn::Arm>) {

    let str_ident = match &member.id {
        swc_ecma_ast::TsEnumMemberId::Ident(ident) => ident.sym.as_str(),
        swc_ecma_ast::TsEnumMemberId::Str(str) => str.value.as_str(),
    };

    let variant = match parse_string!(str_ident => syn::Variant) {
        Ok(ok) => ok,
        Err(err) => abort!(err),
    };

    let arm = member
        .init
        .as_ref()
        .and_then(|expr| expr.as_lit())
        .and_then(|lit| lit.as_str())
        .and_then(|lit| Some(lit.value.as_str()))
        .and_then(|str| parse_quote!({#ident::#variant => #str} as syn::Arm).ok());

    arm.is_none()
        .then(|| emit_call_site_warning!("That type of enum is not implemented"));

    (variant, arm)
}

pub fn ts_enum_to_token_stream(decl: &TsEnumDecl, cm: &SourceMap) -> proc_macro::TokenStream {
    let ident: syn::Ident = safe_convert_ident(&decl.id, cm);

    let (variants, match_arms): (Vec<syn::Variant>, Vec<Option<syn::Arm>>) = decl
        .members
        .iter()
        .map(|m| enum_member_to_match_arm(&ident, m, cm))
        .unzip();

    let match_arms = match_arms.into_iter().collect::<Option<Vec<_>>>();

    let mut declaration = quote::quote! {
      pub enum #ident {
        #(#variants),*
      }
    };

    if let Some(match_arms) = match_arms {
        declaration.extend(quote::quote! {
          impl #ident {
            fn as_string(&self) -> &str {
                match self {
                    #(#match_arms),*,
                    _ => panic!("This variant can not be converted to a string"),
                }
            }
          }

          impl Into<crate::types::Any> for #ident{
            fn into(self) -> crate::types::Any {
                Into::<crate::types::String>::into(self.as_string()).into()
            }
          }
        });
    }

    declaration.into()
}
