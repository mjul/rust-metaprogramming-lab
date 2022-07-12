use proc_macro::TokenStream;
use quote::quote;

pub fn generate_code_for_meta_model(ast: metamodel::Expr) -> TokenStream {
    println!("🚀🚀🚀 meta-model: {:?}", ast);

    let code = match ast {
        metamodel::Expr::RecordDeclarationExpr(rd) => match rd {
            metamodel::RecordDeclaration {
                name,
                documentation,
                fields,
            } => match name {
                metamodel::Name::Literal(name) => {
                    let struct_ident: syn::Ident = syn::parse_str(&name).unwrap();
                    struct FieldInfo {
                        ident: syn::Ident,
                        ty: syn::Type,
                        field: syn::Field,
                    }
                    let field_infos: Vec<FieldInfo> = fields
                        .iter()
                        .map(|fd| {
                            let metamodel::Name::Literal(field_name) = &fd.name;
                            let field_ident: syn::Ident =
                                syn::parse_str(field_name.as_str()).unwrap();
                            let field_type = match &fd.field_type {
                                // pick the corresponding Rust data types
                                metamodel::Type::Primitive(metamodel::PrimitiveType::Id) => "u64",
                                metamodel::Type::Primitive(metamodel::PrimitiveType::LocalDate) => "String",
                                metamodel::Type::Primitive(metamodel::PrimitiveType::String) => "String",
                            };
                            let field_type_path = syn::Type::Path(syn::TypePath {
                                qself: None,
                                path: syn::parse_str(field_type).unwrap(),
                            });

                            let field: syn::Field = syn::Field {
                                attrs: vec![],
                                vis: syn::Visibility::Public(syn::VisPublic {
                                    pub_token: syn::token::Pub::default(),
                                }),
                                ident: Some(field_ident.clone()),
                                colon_token: Some(syn::token::Colon::default()),
                                ty: field_type_path.clone(),
                            };

                            FieldInfo {
                                ident: field_ident,
                                ty: field_type_path,
                                field,
                            }
                        })
                        .collect();

                    // Fields
                    let mut pnames: syn::punctuated::Punctuated<syn::Field, syn::Token![,]> =
                        syn::punctuated::Punctuated::new();
                    for fi in field_infos.iter() {
                        pnames.push(fi.field.clone());
                    }
                    let struct_fields = syn::Fields::Named(syn::FieldsNamed {
                        brace_token: syn::token::Brace::default(),
                        named: pnames,
                    });

                    let mut new_inputs: syn::punctuated::Punctuated<syn::FnArg, syn::Token![,]> =
                        syn::punctuated::Punctuated::new();
                    for fi in field_infos.iter() {
                        new_inputs.push(syn::FnArg::Typed(syn::PatType {
                            attrs: vec![],
                            pat: Box::new(syn::Pat::Ident(syn::PatIdent {
                                attrs: vec![],
                                by_ref: None,
                                mutability: None,
                                ident: fi.ident.clone(),
                                subpat: None,
                            })),
                            colon_token: syn::token::Colon::default(),
                            ty: Box::new(fi.ty.clone()),
                        }))
                    }

                    let mut new_struct_fields: syn::punctuated::Punctuated<
                        syn::FieldValue,
                        syn::Token![,],
                    > = syn::punctuated::Punctuated::new();
                    for fi in field_infos.iter() {
                        let mut ps: syn::punctuated::Punctuated<syn::PathSegment, syn::Token![::]> =
                            syn::punctuated::Punctuated::new();
                        ps.push(syn::PathSegment {
                            ident: fi.ident.clone(),
                            arguments: syn::PathArguments::None,
                        });
                        new_struct_fields.push(syn::FieldValue {
                            attrs: vec![],
                            member: syn::Member::Named(fi.ident.clone()),
                            colon_token: Some(syn::token::Colon::default()),
                            expr: syn::Expr::Path(syn::ExprPath {
                                attrs: vec![],
                                qself: None,
                                path: syn::Path {
                                    leading_colon: None,
                                    segments: ps,
                                },
                            }),
                        });
                    }

                    let record_doc_label = documentation.label;
                    let record_doc_description = documentation.description;

                    quote!(
                            struct #struct_ident #struct_fields

                            impl #struct_ident {
                                pub fn new(#new_inputs) -> Self { Self { #new_struct_fields } }
                            }

                            impl Into<metamodel::Displayable> for #struct_ident {
                                fn into(self) -> metamodel::Displayable {
                                    metamodel::Displayable {
                                        documentation: Documentation::new(#record_doc_label, #record_doc_description),
                                        values: vec![]
                                    }
                                }
                            }
                    )
                }
            },
        },
        _ => todo!(),
    };

    println!("🚀🚀🚀 code: {}", code.to_string());
    
    code.into()
}

#[cfg(test)]
mod playground_tests {
    #[test]
    fn new() {
        struct Foo {
            a: i32,
        }

        let ast: syn::ItemFn = syn::parse_str("pub fn Foo(x: i32) -> Foo { Foo { a:x } }").unwrap();
        dbg!(ast);
    }
}


