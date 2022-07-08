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

/// Parse a Tuple expressions that has a string and a map as an Array of (key,value)-pairs)
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

/// Parse an Array with a name and an documentation tuple, e.g. [("name", "foo"), ("documentation", [("label", "Foo"), ("description", "Description of Foo")])]
fn parse_array_with_name_and_documentation_tuple(
    expr: &syn::Expr,
) -> Option<(String, HashMap<String, String>)> {
    match expr {
        syn::Expr::Array(a) => match a {
            syn::ExprArray {
                attrs,
                bracket_token,
                elems,
            } => {
                println!("🚀🚀🚀 parsing an ARRAY with name and documentation!");
                let mut ai = elems.iter();
                match (ai.next(), ai.next(), ai.next()) {
                    (Some(head), Some(tail), None) => {
                        let name_pair = parse_tuple_string_string(head);
                        println!("🚀🚀🚀 name_pair: {:?}", name_pair);
                        let name = match name_pair {
                            Some(np) => get_named_value_from_pair("name", np),
                            None => None,
                        };
                        println!("🚀🚀🚀 name: {:?}", name);

                        println!("🚀🚀🚀 tail: {:?}", tail);

                        let docs_pair = parse_tuple_string_map(tail);
                        println!("🚀🚀🚀 docs_pair: {:?}", docs_pair);
                        let docs_map = match docs_pair {
                            Some(dp) => get_named_value_from_pair("documentation", dp),
                            None => todo!(),
                        };
                        println!("🚀🚀🚀 docs_map: {:?}", docs_map);

                        match (name, docs_map) {
                            (Some(n), Some(dm)) => Some((n, dm)),
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

/// Parse a tuple of (tag, string) where the tag must match the given tag.
fn parse_tuple_with_tagged_string(tag: &str, expr: &syn::Expr) -> Option<String> {
    match parse_tuple_string_string(&expr) {
        Some((k, v)) => {
            if tag == k {
                Some(v)
            } else {
                None
            }
        }
        _ => todo!("not a tuple of (key,value) strings!"),
    }
}

fn parse_documentation_tuple(expr: &syn::Expr) -> Option<String> {
    todo!()
}

fn parse_tuple_with_string_and_array_of_name_and_documentation_tuples(
    expr: &syn::Expr,
) -> Option<(String, Vec<(String, HashMap<String, String>)>)> {
    match expr {
        syn::Expr::Tuple(syn::ExprTuple {
            attrs: _,
            paren_token: _,
            elems,
        }) => {
            let mut i = elems.iter();
            match (i.next(), i.next(), i.next()) {
                (Some(tag_expr), Some(array_expr), None) => {
                    println!("🚀🚀🚀 tag_expr    = {:?}", tag_expr);
                    let opt_tag = parse_lit_string(tag_expr);
                    match opt_tag {
                        Some(tag) => {
                            match array_expr {
                                syn::Expr::Array(ae) => match ae {
                                    syn::ExprArray {
                                        attrs: _,
                                        bracket_token: _,
                                        elems,
                                    } => {
                                        let arr = vec![];
                                        for field_expr in elems.iter() {
                                            match field_expr {
                                                syn::Expr::Tuple(syn::ExprTuple {
                                                    attrs: _,
                                                paren_token: _,
                                                elems,
                                                }) => {
                                                    let mut fi = elems.iter();
                                                    match (fi.next(), fi.next(), fi.next(), fi.next()) {
                                                        (Some(name_expr), Some(docs_expr), Some(type_expr), None) => {
                                                            let opt_name_kv = parse_tuple_with_tagged_string("name", &name_expr);
                                                            let opt_docs = parse_documentation_tuple(&docs_expr);
                                                            let opt_type = parse_tuple_with_tagged_string("type", &type_expr);
                                                        },
                                                        _ => todo!()
                                                    }
                                                },
                                                _ => todo!("exmpected a tuple for each field")
                                            }
                                            // let ((name_key,name),(docs_key, docs),(type_key, ty)) = parse_array_of_string_string_tuple_and_string_map_tuple_and_string_string_tuple(&field_expr);
                                            // TODO: assert correct keys
                                            // arr.push((name,docs,ty));
                                        }
                                        Some((tag.to_string(), arr))
                                    },
                                },
                                _ => todo!("expected an array expression following the tag"),
                            }
                        },
                        None => todo!("expected a literal string tag as first element of tuple: (tag, [name-doc-tuples...])"),
                    }
                }
                _ => todo!("expected tuple of two elements: (tag, [name-doc-tuples...])"),
            }
        }
        _ => None,
    }
}

fn generate_code_for_meta_model(ast: metamodel::Expr) -> TokenStream {
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

/// Create a metamodel Name object from a string
fn to_metamodel_name(name: &str) -> metamodel::Name {
    metamodel::Name::Literal(name.to_string())
}

/// Create the metamodel Documentation object from a map of key-values
fn to_metamodel_documentation(dm: &HashMap<String, String>) -> metamodel::Documentation {
    metamodel::Documentation::new(dm.get("label").unwrap(), dm.get("description").unwrap())
}

/// Create a metamodel Type object from a string
fn to_metamodel_type(type_name: &str) -> metamodel::Type {
    match type_name {
        "ID" => metamodel::Type::Primitive(metamodel::PrimitiveType::Id),
        "LocalDate" => metamodel::Type::Primitive(metamodel::PrimitiveType::LocalDate),
        _ => todo!("unknown type"),
    }
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
            match (i.next(), i.next(), i.next(), i.next()) {
                (Some(tag_expr), expr1, expr2, expr3) => {
                    println!("🚀🚀🚀 tag_expr    = {:?}", tag_expr);
                    let opt_tag = parse_lit_string(tag_expr);
                    match opt_tag {
                        Some(tag) => match tag.as_str() {
                            "record" => {
                                println!("🚀🚀🚀 compiling a RECORD type!");
                                match (expr1, expr2, expr3) {
                                    (Some(name_expr),Some(fields_expr), None) => {
                                        match parse_array_with_name_and_documentation_tuple(&name_expr) {
                                            Some((n, dm)) => {
                                                let opt_fields = parse_tuple_with_string_and_array_of_name_and_documentation_tuples(&fields_expr);

                                                println!("🚀🚀🚀 compiling a RECORD type with fields: {:?}", opt_fields);
                                                let fields = match opt_fields {
                                                    Some((tag, field_name_docs)) => {
                                                        match tag.as_str() {
                                                            "fields" => {
                                                                println!("🚀🚀🚀 compiling fields: {:?}", field_name_docs);
                                                                let mut fields : Vec<metamodel::FieldDeclaration> = vec![];
                                                                for (name, docs) in field_name_docs.iter() {
                                                                    let ty = "ID";
                                                                    println!("🚀🚀🚀 compiling field: {:?} {:?} {:?}", name, docs, ty);
                                                                    let fd = metamodel::FieldDeclaration::new(
                                                                            to_metamodel_name(name),
                                                                            to_metamodel_documentation(&docs),
                                                                            to_metamodel_type(&ty),
                                                                            );
                                                                    fields.push(fd);
                                                                }
                                                                fields
                                                            },
                                                            _ => todo!("incorrect tag, expected 'fields'"),
                                                        }},
                                                    _ => todo!("invalid fields declaration"),
                                                };

                                                metamodel::Expr::RecordDeclarationExpr(
                                                        metamodel::RecordDeclaration::new(
                                                            to_metamodel_name(&n),
                                                            to_metamodel_documentation(&dm),
                                                            fields,
                                                        ),
                                                )
                                            },
                                            None => todo!("could not parse name and documentation tuple"),
                                        }
                                    },
                                    (Some(_),None, None) => todo!("invalid record declaration (fields missing), expected tuple of three elements, (tag, [name...], (fields [...]))"),
                                    _ => todo!("invalid record declaration, expected tuple of three elements, (tag, [name...], (fields [...]))"),
                                }
                                // end "record"
                            }
                            _ => todo!("unknown tag"),
                        },
                        None => {
                            todo!("No literal string tag found as first element of first tuple.")
                        }
                    }
                }
                (None, _, _, _) => todo!("Expected non-empty tuple"),
            }
        }
        _ => todo!("expected a tuple"),
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
    use syn::parse_str;
    #[test]
    fn expr() {
        let es: syn::ExprStruct = syn::parse_str("Foo { a: i32, b : usize }").unwrap();
        dbg!(es);
        //let p : syn::Path = syn::parse_str("foobar").unwrap();
        //dbg!(p);
        let fv: syn::FieldValue = syn::parse_str("id: i32").unwrap();
        dbg!(fv);
    }
}
