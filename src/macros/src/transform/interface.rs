use swc_ecma_ast::{
    TsCallSignatureDecl, TsConstructSignatureDecl, TsGetterSignature, TsIndexSignature,
    TsInterfaceDecl, TsMethodSignature, TsPropertySignature, TsSetterSignature, TsTypeElement,
};


use super::function::method_signature_to_impl_item_fn;

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

pub fn interface_to_token_stream(decl: &TsInterfaceDecl) -> proc_macro::TokenStream {
    let ident: syn::Ident = syn::parse_str(&format!("{}{}", "", decl.id.sym.as_str())).expect("");
    let TypeElements { methods, .. } =
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
                .map(|(i, method)| syn::ImplItemFn {
                    attrs: method.attrs,
                    vis: method.vis,
                    defaultness: method.defaultness,
                    sig: syn::Signature {
                        constness: method.sig.constness,
                        asyncness: method.sig.asyncness,
                        unsafety: method.sig.unsafety,
                        abi: method.sig.abi,
                        fn_token: method.sig.fn_token,
                        ident: syn::Ident::new(
                            &format!("{}{}", method.sig.ident.to_string(), i),
                            method.sig.ident.span(),
                        ),
                        generics: method.sig.generics,
                        paren_token: method.sig.paren_token,
                        inputs: method.sig.inputs,
                        variadic: method.sig.variadic,
                        output: method.sig.output,
                    },
                    block: method.block,
                })
                .collect(),
        })
        .collect();

    quote::quote! {
        pub struct #ident{
            _self: crate::Object
        }
        impl #ident {
            #(#methods)*
        }
    }.into()
}
