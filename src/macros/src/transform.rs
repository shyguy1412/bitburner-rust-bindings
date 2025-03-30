use proc_macro::TokenStream;
use quote::quote;
use swc_ecma_ast::{Decl, TsEnumDecl, TsInterfaceDecl, TsTypeAliasDecl};

pub fn declaration_to_struct(decl: Decl) -> TokenStream {
    match decl {
        Decl::TsInterface(decl) => interface_to_struct(*decl),
        Decl::TsTypeAlias(decl) => type_alias_to_struct(*decl),
        Decl::TsEnum(decl) => enum_to_struct(*decl),
        _ => TokenStream::new(),
    }
}

fn interface_to_struct(decl: TsInterfaceDecl) -> TokenStream {
    let ident: syn::Ident =
        syn::parse_str(&format!("{}{}", "Interface", decl.id.sym.as_str())).expect("");
    quote! {
      pub struct #ident{

      }
    }
    .into()
}

fn type_alias_to_struct(decl: TsTypeAliasDecl) -> TokenStream {
    let ident: syn::Ident =
        syn::parse_str(&format!("{}{}", "Type", decl.id.sym.as_str())).expect("");

    quote! {
        //TYPE
        pub struct #ident{

        }
    }
    .into()
}

fn enum_to_struct(decl: TsEnumDecl) -> TokenStream {
    let ident: syn::Ident = syn::parse_str(decl.id.sym.as_str()).expect("");
    let (variants, match_arms): (Vec<syn::Variant>, Vec<Option<syn::Arm>>) = decl
        .members
        .iter()
        .map(|member| {
            let variant =
                syn::parse_str::<syn::Variant>(member.id.as_ident().expect("").sym.as_str())
                    .expect("");

            let init = member
                .init
                .clone()
                .and_then(|expr| expr.lit())
                .and_then(|lit| lit.str())
                .and_then(|lit| Some(lit.value.as_str().to_owned()))
                .and_then(|str| {
                    Some(
                        syn::parse::<syn::Arm>(
                            quote! {
                              #ident::#variant => #str
                            }
                            .into(),
                        )
                        .expect(""),
                    )
                });

            (variant, init)
        })
        .unzip();

    let match_arms = match_arms.into_iter().collect::<Option<Vec<_>>>();

    let mut declaration = quote! {
      pub enum #ident {
        #(#variants),*
      }
    };

    if let Some(match_arms) = match_arms {
        declaration.extend(quote! {
          impl #ident {
            fn as_string(&self) -> &str {
                match self {
                    #(#match_arms),*,
                    _ => panic!("This variant can not be converted to a string"),
                }
            }
          }

          impl Into<Any> for #ident{
            fn into(self) -> Any {
                Into::<self::types::String>::into(self.as_string()).into()
            }
          }
        });
    }

    declaration.into()
}
