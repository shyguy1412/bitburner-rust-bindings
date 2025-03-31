use swc_ecma_ast::TsTypeAliasDecl;

pub fn type_alias_to_struct(decl: TsTypeAliasDecl) -> proc_macro::TokenStream {
  let ident: syn::Ident = syn::parse_str(&format!("{}{}", "", decl.id.sym.as_str())).expect("");

  quote::quote! {
      //TYPE
      pub struct #ident{

      }
  }
  .into()
}
