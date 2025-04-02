use std::collections::HashMap;

use swc_ecma_ast::{
    TsCallSignatureDecl, TsConstructSignatureDecl, TsGetterSignature, TsIndexSignature,
    TsInterfaceDecl, TsMethodSignature, TsPropertySignature, TsSetterSignature, TsTypeElement,
};

use crate::transform::safe_convert_ident;

use super::parse_quote;

struct TypeElements {
    constructors: Vec<TsConstructSignatureDecl>,
    getters: Vec<TsGetterSignature>,
    setters: Vec<TsSetterSignature>,
    props: Vec<TsPropertySignature>,
    index_props: Vec<TsIndexSignature>,
    methods: Vec<TsMethodSignature>,
    call: Vec<TsCallSignatureDecl>,
}

impl TypeElements {
    fn new() -> Self {
        TypeElements {
            constructors: vec![],
            getters: vec![],
            setters: vec![],
            props: vec![],
            index_props: vec![],
            methods: vec![],
            call: vec![],
        }
    }
}

pub fn interface_to_struct(decl: TsInterfaceDecl) -> proc_macro::TokenStream {
    let ident: syn::Ident = syn::parse_str(&format!("{}{}", "", decl.id.sym.as_str())).expect("");
    let TypeElements { props, methods, .. } =
        decl.body
            .body
            .iter()
            .fold(TypeElements::new(), |mut prev, m| {
                match m {
                    TsTypeElement::TsCallSignatureDecl(node) => prev.call.push(node.clone()),
                    TsTypeElement::TsConstructSignatureDecl(node) => {
                        prev.constructors.push(node.clone())
                    }
                    TsTypeElement::TsPropertySignature(node) => prev.props.push(node.clone()),
                    TsTypeElement::TsGetterSignature(node) => prev.getters.push(node.clone()),
                    TsTypeElement::TsSetterSignature(node) => prev.setters.push(node.clone()),
                    TsTypeElement::TsMethodSignature(node) => prev.methods.push(node.clone()),
                    TsTypeElement::TsIndexSignature(node) => prev.index_props.push(node.clone()),
                };
                prev
            });

    //I dont hate myself :D
    let methods: Vec<_> = methods
        .iter()
        .map(method_signature_to_impl_item_fn)
        .collect::<Vec<syn::ImplItemFn>>()
        .chunk_by(|prev, cur| prev.sig.ident.to_string() == cur.sig.ident.to_string())
        .flat_map(|methods| match methods.len() {
            0 => panic!("WHAT?? D:"),
            1 => methods.to_vec(),
            _ => methods
                .to_vec()
                .into_iter()
                .enumerate()
                .map(|(i, mut method)| {
                    let str = method.sig.ident.to_string();
                    method.sig.ident =
                        syn::Ident::new(&format!("{}{}", str, i), method.sig.ident.span());
                    method
                })
                .collect(),
        })
        .collect();

    let declaration = quote::quote! {
        pub struct #ident{
            _self: crate::Object
        }
        impl #ident {
            #(#methods)*
        }
    };

    declaration.into()
}

fn method_signature_to_impl_item_fn(method: &TsMethodSignature) -> syn::ImplItemFn {
    let ident = method
        .key
        .as_ident()
        .and_then(safe_convert_ident)
        .expect("computed method signatures are not supported yet");

    parse_quote!({
        pub fn #ident(){} 
    } as syn::ImplItemFn => "")
}
