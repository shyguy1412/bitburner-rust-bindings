use swc_ecma_ast::{TsEnumDecl, TsEnumMember};

use super::{parse_quote, safe_convert_ident};

fn enum_member_to_match_arm(
    member: (&syn::Ident, &TsEnumMember),
) -> (syn::Variant, Option<syn::Arm>) {
    let (ident, member) = member;
    let variant =
        syn::parse_str::<syn::Variant>(member.id.as_ident().expect("").sym.as_str()).expect("");

    let arm = member
        .init
        .clone()
        .and_then(|expr| expr.lit())
        .and_then(|lit| lit.str())
        .and_then(|lit| Some(lit.value.as_str().to_owned()))
        .and_then(|str| Some(parse_quote!({#ident::#variant => #str} as syn::Arm => "")));
    (variant, arm)
}

pub fn ts_enum_to_token_stream(decl: &TsEnumDecl) -> proc_macro::TokenStream {
    let ident: syn::Ident = safe_convert_ident(&decl.id).unwrap();
    
    let (variants, match_arms): (Vec<syn::Variant>, Vec<Option<syn::Arm>>) = decl
        .members
        .iter()
        .map(|m| (&ident, m))
        .map(enum_member_to_match_arm)
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
