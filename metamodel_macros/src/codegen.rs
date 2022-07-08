use proc_macro::TokenStream;
use quote::quote;

pub fn generate_code_for_meta_model(ast: metamodel::Expr) -> TokenStream {
    println!("🚀🚀🚀 meta-model: {:?}", ast);

    let code = match ast {
        metamodel::Expr::RecordDeclarationExpr(rd) => match rd {
            metamodel::RecordDeclaration {
                name,
                documentation: _,
                fields,
            } => match name {
                metamodel::Name::Literal(name) => {
                    let struct_ident: syn::Ident = syn::parse_str(&name).unwrap();
                    let struct_path_ident: syn::Path = syn::parse_str(&name).unwrap();

                    // Fields
                    let mut pnames: syn::punctuated::Punctuated<syn::Field, syn::Token![,]> =
                        syn::punctuated::Punctuated::new();
                    for fd in fields.iter() {
                        let metamodel::Name::Literal(field_name) = &fd.name;
                        let field_type = match &fd.field_type {
                            // pick the corresponding Rust data types
                            metamodel::Type::Primitive(metamodel::PrimitiveType::Id) => "u64",
                            metamodel::Type::Primitive(metamodel::PrimitiveType::LocalDate) => {
                                "String"
                            }
                        };

                        // experimental
                        let field_ident_path: syn::Path = syn::parse_str("foobar").unwrap();
                        let field_expr_path: syn::ExprPath = syn::parse_str("i32").unwrap();
                        let i32_path: syn::Path = syn::parse_str("i32").unwrap();

                        // build FieldValue instance  { attrs, member, colon_token, expr }
                        let fv_member_ident: syn::Ident =
                            syn::parse_str(field_name.as_str()).unwrap();
                        //let fv_member = syn::Member::Named(fv_member_ident);
                        let fv_colon = Some(syn::token::Colon::default());
                        let fv_expr_path: syn::Path = syn::parse_str(field_type).unwrap();
                        //let fv_expr = syn::Expr::Path(syn::ExprPath {attrs:  vec![], qself: None, path: fv_expr_path});
                        //let fv = syn::FieldValue { attrs: vec![], member:fv_member, colon_token: fv_colon, expr: fv_expr};

                        let field: syn::Field = syn::Field {
                            attrs: vec![],
                            vis: syn::Visibility::Public(syn::VisPublic {
                                pub_token: syn::token::Pub::default(),
                            }),
                            ident: Some(fv_member_ident),
                            colon_token: fv_colon,
                            ty: syn::Type::Path(syn::TypePath {
                                qself: None,
                                path: fv_expr_path,
                            }),
                        };
                        pnames.push(field);
                    }
                    let struct_fields = syn::Fields::Named(syn::FieldsNamed {
                        brace_token: syn::token::Brace::default(),
                        named: pnames,
                    });

                    //let struct_expr = syn::ExprStruct { attrs: vec![], path: struct_path_ident, brace_token: syn::token::Brace::default(), fields: struct_fields };
                    quote!( struct #struct_ident #struct_fields )
                }
            },
        },
        _ => todo!(),
    };

    println!("🚀🚀🚀 code: {:?}", code);

    code.into()
}
