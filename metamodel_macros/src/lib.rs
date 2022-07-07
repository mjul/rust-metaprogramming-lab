extern crate metamodel;

use proc_macro::{self, TokenStream};
use quote::quote;
use std::collections::HashMap;
use std::fmt;
use syn::parse::Parse;
use syn::{parse_macro_input, Result, Token};

#[derive(Debug)]
struct GenStructsInputEnum {}

impl Parse for GenStructsInputEnum {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        todo!()
    }
}

#[derive(Debug)]
enum GenStructsInput {
    Enum(GenStructsInputEnum),
}

impl Parse for GenStructsInput {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Token![enum]) {
            input.parse().map(GenStructsInput::Enum)
        } else {
            Err(lookahead.error())
        }
    }
}

/// Generate Rust data structures from a meta-model
#[proc_macro]
pub fn generate_data_structures(input: TokenStream) -> TokenStream {
    let item: syn::Expr = syn::parse(input).expect("failed to parse input");
    println!("🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀 running macro: generate_data_structures...");
    println!("🚀🚀🚀 INPUT: {:#?}", item);

    let model_identifier = match item {
        syn::Expr::Call(c) => match c {
            syn::ExprCall {
                attrs,
                func,
                args,
                paren_token,
            } => match *func {
                syn::Expr::Path(p) => match p {
                    syn::ExprPath { attrs, qself, path } => {
                        println!("Call -> ExprCall -> Path -> ExprPath");
                        None
                    }
                },
                _ => todo!(),
            },
        },
        // For now, let's just assume that people pass an identifier with the meta-model as its value.
        syn::Expr::Path(p) => match p {
            syn::ExprPath {
                attrs: _,
                qself: None,
                path,
            } => match path {
                syn::Path {
                    leading_colon: None,
                    segments,
                } => match segments.len() {
                    1 => match segments.first() {
                        Some(syn::PathSegment {
                            arguments: syn::PathArguments::None,
                            ident,
                        }) => Some(ident.clone()),
                        None => todo!(),
                        _ => todo!(),
                    },
                    _ => todo!(),
                },
                _ => todo!(),
            },
            _ => todo!(),
        },
        _ => {
            println!("🚀🚀🚀 no match for Expr {:?}", item);
            None
        }
    };

    println!("🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀 macro input parsing completed...");

    // TODO: generate some real code, this is just a placeholder
    quote! {
        struct Foo { id: usize};
        impl Foo { pub fn new(id:usize) -> Self {
            Self { id }
        }
    } }
    .into()
}

fn parse_lit_string(lit: &syn::Expr) -> Option<String> {
    match lit {
        syn::Expr::Lit(el) => match el {
            syn::ExprLit { attrs: _, lit } => match lit {
                syn::Lit::Str(s) => Some(s.value()),
                _ => todo!(),
            },
        },
        _ => todo!(),
    }
}

fn parse_tuple_string_string(tup: &syn::Expr) -> Option<(String, String)> {
    match tup {
        syn::Expr::Tuple(te) => match te {
            syn::ExprTuple {
                paren_token: _,
                attrs: _,
                elems,
            } => {
                let mut i = elems.iter();
                match (i.next(), i.next(), i.next()) {
                    (Some(key_expr), Some(val_expr), None) => {
                        let k = parse_lit_string(key_expr);
                        let v = parse_lit_string(val_expr);
                        match (k, v) {
                            (Some(key), Some(val)) => Some((key, val)),
                            _ => None,
                        }
                    }
                    _ => todo!(),
                }
            }
        },
        _ => todo!(),
    }
}

fn parse_array_of_string_string_tuples(arr: &syn::Expr) -> Option<Vec<(String, String)>> {
    match arr {
        syn::Expr::Array(ae) => match ae {
            syn::ExprArray {
                attrs: _,
                bracket_token: _,
                elems,
            } => Some(
                elems
                    .iter()
                    .map(|e| parse_tuple_string_string(e).expect("expected (key, value) pair"))
                    .collect(),
            ),
        },
        _ => todo!(),
    }
}

fn parse_tuple_string_map(tup: &syn::Expr) -> Option<(String, HashMap<String, String>)> {
    match tup {
        syn::Expr::Tuple(te) => match te {
            syn::ExprTuple {
                paren_token: _,
                attrs: _,
                elems,
            } => {
                let mut i = elems.iter();
                match (i.next(), i.next(), i.next()) {
                    (Some(key_expr), Some(val_expr), None) => {
                        let key_str = parse_lit_string(key_expr);
                        let val_pairs = parse_array_of_string_string_tuples(val_expr);
                        let val_map = match val_pairs {
                            Some(vps) => {
                                let mut result: HashMap<String, String> = HashMap::new();
                                for (k, v) in vps.iter() {
                                    // TODO: check for duplicates
                                    result.insert(k.to_string(), v.to_string());
                                }
                                Some(result)
                            }
                            _ => None,
                        };
                        match (key_str, val_map) {
                            (Some(key), Some(val)) => Some((key, val)),
                            _ => None,
                        }
                    }
                    _ => todo!(),
                }
            }
        },
        _ => todo!(),
    }
}

/// Given a (key, value) pair as a tuple, extract the value if the key matches the expected tag value.
fn get_named_value_from_pair<T>(expected_tag: &str, (k, v): (String, T)) -> Option<T> {
    if expected_tag == k.as_str() {
        Some(v)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    mod get_named_value_from_pair_tests {
        use super::super::*;

        #[test]
        fn for_matching_tag() {
            let actual =
                get_named_value_from_pair("name", (String::from("name"), String::from("value")));
            assert_eq!(Some(String::from("value")), actual);
        }

        #[test]
        fn for_other_tag() {
            let actual = get_named_value_from_pair(
                "not-this-tag",
                (String::from("name"), String::from("value")),
            );
            assert_eq!(None, actual);
        }
    }
}


fn generate_code_for_meta_model(ast : metamodel::Expr) -> TokenStream {
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
                    let struct_path_ident : syn::Path = syn::parse_str(&name).unwrap();
                    let struct_fields: syn::Fields = match fields.is_empty() {
                        true => syn::Fields::Unit,
                        false => {
                            let mut pnames : syn::punctuated::Punctuated<syn::Field, syn::Token![,]> = syn::punctuated::Punctuated::new();
                            for fd in fields.iter() {
                                // TODO: take the field name
                                let field_ident_path : syn::Path = syn::parse_str("foobar").unwrap();
                                let field_expr_path : syn::ExprPath = syn::parse_str("i32").unwrap();
                                let i32_path : syn::Path = syn::parse_str("i32").unwrap();

                                // build FieldValue instance  { attrs, member, colon_token, expr }
                                let fv_member_ident : syn::Ident = syn::parse_str("x").unwrap();
                                //let fv_member = syn::Member::Named(fv_member_ident);
                                let fv_colon = Some(syn::token::Colon::default());
                                let fv_expr_path : syn::Path = syn::parse_str("i32").unwrap();
                                //let fv_expr = syn::Expr::Path(syn::ExprPath {attrs:  vec![], qself: None, path: fv_expr_path});
                                //let fv = syn::FieldValue { attrs: vec![], member:fv_member, colon_token: fv_colon, expr: fv_expr};

                                let field : syn::Field = syn::Field { attrs:vec![],
                                    vis:syn::Visibility::Public(syn::VisPublic {pub_token: syn::token::Pub::default()}),
                                    ident: Some(fv_member_ident),
                                    colon_token: fv_colon,
                                    ty: syn::Type::Path(syn::TypePath { qself: None, path:fv_expr_path }),
                                };
                                pnames.push(field);

                            }
                            syn::Fields::Named(syn::FieldsNamed { brace_token: syn::token::Brace::default(), named: pnames })
                        },
                    };
                    //let struct_expr = syn::ExprStruct { attrs: vec![], path: struct_path_ident, brace_token: syn::token::Brace::default(), fields: struct_fields };
                    quote!( struct #struct_ident { #struct_fields } )
                }
            },
        },
        _ => todo!(),
    };

    println!("🚀🚀🚀 code: {:?}", code);

    code.into()
}

#[proc_macro]
pub fn generate_model_from_tuple(input: TokenStream) -> TokenStream {
    let item: syn::Expr = syn::parse(input).expect("failed to parse input");
    println!("🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀 running macro: generate_model_from_tuple...");
    println!("🚀🚀🚀 INPUT: {:#?}", item);

    let ast = match item {
        syn::Expr::Tuple(syn::ExprTuple {
            attrs: _,
            paren_token: _,
            elems,
        }) => {
            let mut i = elems.iter();
            match (i.next(), i.next(), i.next()) {
                (Some(head), Some(tail), None) => {
                    println!("head = {:?}", head);
                    println!("tail = {:?}", tail);
                    let opt_tag = parse_lit_string(head);
                    match opt_tag {
                        Some(tag) => match tag.as_str() {
                            "record" => {
                                println!("🚀🚀🚀 compiling a RECORD type!");
                                match tail {
                                    syn::Expr::Array(a) => match a {
                                        syn::ExprArray {
                                            attrs,
                                            bracket_token,
                                            elems,
                                        } => {
                                            println!("🚀🚀🚀 compiling an ARRAY!");
                                            let mut ai = elems.iter();
                                            match (ai.next(), ai.next(), ai.next()) {
                                                (Some(head), Some(tail), None) => {
                                                    let name_pair = parse_tuple_string_string(head);
                                                    println!("🚀🚀🚀 name_pair: {:?}", name_pair);
                                                    let name = match name_pair {
                                                        Some(np) => {
                                                            get_named_value_from_pair("name", np)
                                                        }
                                                        None => None,
                                                    };
                                                    println!("🚀🚀🚀 name: {:?}", name);

                                                    println!("🚀🚀🚀 tail: {:?}", tail);

                                                    let docs_pair = parse_tuple_string_map(tail);
                                                    println!("🚀🚀🚀 docs_pair: {:?}", docs_pair);
                                                    let docs_map = match docs_pair {
                                                        Some(dp) => get_named_value_from_pair(
                                                            "documentation",
                                                            dp,
                                                        ),
                                                        None => todo!(),
                                                    };
                                                    println!("🚀🚀🚀 docs_map: {:?}", docs_map);

                                                    match (name, docs_map) {
                                                        (Some(n), Some(dm)) => {
                                                            let no_fields: Vec<
                                                                metamodel::FieldDeclaration,
                                                            > = vec![];

                                                            let one_field = vec![metamodel::FieldDeclaration::new(metamodel::Name::Literal(String::from("id")),
                                                            metamodel::Documentation::new("ID", "The entity ID."),
                                                            metamodel::Type::Primitive(metamodel::PrimitiveType::Id))];

                                                            metamodel::Expr::RecordDeclarationExpr(
                                                                metamodel::RecordDeclaration::new(
                                                                    metamodel::Name::Literal(n),
                                                                    metamodel::Documentation::new(
                                                                        dm.get("label").unwrap(),
                                                                        dm.get("description")
                                                                            .unwrap(),
                                                                    ),
                                                                    no_fields,
                                                                    //one_field,
                                                                ),
                                                            )
                                                        }
                                                        _ => todo!(),
                                                    }
                                                }
                                                _ => todo!(),
                                            }
                                        }
                                    },
                                    _ => todo!(),
                                }
                            }
                            _ => todo!("unknown tag"),
                        },
                        None => todo!("No tag found as first element of first tuple."),
                    }
                }
                _ => todo!(),
            }
        }
        _ => todo!(),
    };

    println!("🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀 macro input parsing completed...");

    generate_code_for_meta_model(ast)
}

/*
    match model_identifier {
        metamodel::Expr::RecordDeclarationExpr { name, documentation } => {
            let sname = "Foo";
            let fdecls = "a : usize";
            quote!(struct #sname {
                #fdecls
            })
        },
        _ => quote!(struct XXXXXX {}),
    }.into()

*/

#[cfg(test)]
mod playground_tests {
    use syn::{parse_str};
    #[test]
    fn expr() {
        let es : syn::ExprStruct = syn::parse_str("Foo { a: i32, b : usize }").unwrap();
        dbg!(es);
        //let p : syn::Path = syn::parse_str("foobar").unwrap();
        //dbg!(p);
        let fv : syn::FieldValue = syn::parse_str("id: i32").unwrap();
        dbg!(fv);
    }
}
