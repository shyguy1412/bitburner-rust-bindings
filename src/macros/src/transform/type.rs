pub fn type_alias_to_token_stream(decl: &TsTypeAliasDecl) -> proc_macro::TokenStream {
    let ident: syn::Ident = syn::parse_str(&format!("{}{}", "", decl.id.sym.as_str())).expect("");

    quote::quote! {
      struct #ident{}
    }
    .into()
}
