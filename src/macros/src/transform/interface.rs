use swc_ecma_ast::{
    TsCallSignatureDecl, TsConstructSignatureDecl, TsGetterSignature, TsIndexSignature,
    TsInterfaceDecl, TsMethodSignature, TsPropertySignature, TsSetterSignature, TsTypeElement,
};

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

    

    let declaration = quote::quote! {
      pub struct #ident{
        _self: crate::Object
      }
    };

    declaration.into()
}
