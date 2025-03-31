use swc_ecma_ast::Decl;

mod r#enum;
pub use r#enum::enum_to_struct;
mod interface;
pub use interface::interface_to_struct;
mod r#type;
pub use r#type::type_alias_to_struct;

pub fn declaration_to_struct(decl: Decl) -> proc_macro::TokenStream {
    match decl {
        Decl::TsInterface(decl) => interface_to_struct(*decl),
        Decl::TsTypeAlias(decl) => type_alias_to_struct(*decl),
        Decl::TsEnum(decl) => enum_to_struct(*decl),
        _ => proc_macro::TokenStream::new(),
    }
}

/**
 * Safely converts TS ident to Rust Ident by prefixing with _ if the ident is a reserved keyword in rust
 */
pub(self) fn safe_convert_ident(ident: &swc_ecma_ast::Ident) -> syn::Ident {
    syn::parse_str::<syn::Ident>(ident.sym.as_str())
        .or(syn::parse_str::<syn::Ident>(&format!(
            "_{}",
            ident.sym.as_str()
        )))
        .expect("")
}

macro_rules! parse_quote {
    ({$($tt:tt)*} as $t:ty => $e:literal) => {
      syn::parse::<$t>(quote::quote!{$($tt)*}.into()).expect($e)
    };
}
pub(crate) use parse_quote;
