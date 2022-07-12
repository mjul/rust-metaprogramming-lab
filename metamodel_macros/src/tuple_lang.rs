extern crate metamodel;

use proc_macro::{self, TokenStream};
use std::collections::HashMap;
use syn::Result;

fn parse_lit_string(lit: &syn::Expr) -> syn::Result<String> {
    match lit {
        syn::Expr::Lit(el) => match el {
            syn::ExprLit { attrs: _, lit } => match lit {
                syn::Lit::Str(s) => syn::Result::Ok(s.value()),
                _ => syn::Result::Err(syn::Error::new_spanned(
                    el,
                    "Expected a Lit str expression.",
                )),
            },
        },
        _ => todo!("parse_lit_string: Expected a Lit expression"),
    }
}

fn parse_tuple_string_string(tup: &syn::Expr) -> syn::Result<(String, String)> {
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
                            (syn::Result::Ok(key), syn::Result::Ok(val)) => {
                                syn::Result::Ok((key, val))
                            }
                            _ => syn::Result::Err(syn::Error::new_spanned(
                                te,
                                "Expected a Tuple of two string literals.",
                            )),
                        }
                    }
                    _ => syn::Result::Err(syn::Error::new_spanned(
                        te,
                        "Expected a Tuple of two strings.",
                    )),
                }
            }
        },
        _ => syn::Result::Err(syn::Error::new_spanned(
            tup,
            "parse_tuple_string_string: expected a Tuple expression",
        )),
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
        _ => todo!("parse_array_of_string_string_tuples: expected an Array"),
    }
}

/// Parse a Tuple expressions that has a string and a map as an Array of (key,value)-pairs)
fn parse_tuple_string_map(tup: &syn::Expr) -> syn::Result<(String, HashMap<String, String>)> {
    match tup {
        syn::Expr::Tuple(te) => {
            match te {
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
                            (syn::Result::Ok(key), Some(val)) => syn::Result::Ok((key, val)),
                            _ => syn::Result::Err(syn::Error::new_spanned(te, "did not find a tuple of a string and an a value map")),
                        }
                    }
                    _ => todo!("parse_tuple_string_map: Expected a string key and a map of string pairs."),
                }
                }
            }
        }
        _ => todo!("parse_array_of_string_string_tuples: Expected a Tuple expression"),
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
) -> syn::Result<(String, HashMap<String, String>)> {
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
                        let name = parse_tuple_with_tagged_string("name", head);
                        println!("🚀🚀🚀 name: {:?}", name);

                        // TODO: use parse_tuple_with_tagged_string(tag, expr)
                        let docs_map = parse_documentation_tuple(&tail);
                        println!("🚀🚀🚀 docs_map: {:?}", docs_map);

                        match (name, docs_map) {
                            (syn::Result::Ok(n), syn::Result::Ok(dm)) => syn::Result::Ok((n, dm)),
                            (syn::Result::Err(e), syn::Result::Ok(_)) => syn::Result::Err(combined_error(head, "Failed to parse name: expected (\"name\", \"value\") pair.", e)),
                            (syn::Result::Ok(_), syn::Result::Err(e)) => syn::Result::Err(combined_error(tail, "Failed to parse documentation: expected (\"documentation\", [...]) pair.", e)),
                            (syn::Result::Err(en), syn::Result::Err(ed)) => {
                                let err_name = combined_error(head, "Failed to parse name: expected (\"name\", \"value\") pair.", en);
                                let err_docs = combined_error(tail, "Failed to parse documentation: expected (\"documentation\", [...]) pair.", ed);
                                let mut combined = syn::Error::new_spanned(expr,  "Failed to parse name and documentation");
                                combined.combine(err_name);
                                combined.combine(err_docs);
                                syn::Result::Err(combined)
                            },
                        }
                    }
                    _ => syn::Result::Err(syn::Error::new_spanned(expr, "Error while parsing array with name and documentation (not a name, documentation array).")),
                }
            }
        },
        _ => todo!("Error while parsing array with name and documentation (not an Array)."),
    }
}

/// Create an error for a syn::Expr that includes an (inner) error. This is useful for providing context for parsing errors.
fn combined_error(expr: &syn::Expr, message: &str, other_error: syn::Error) -> syn::Error {
    let mut err = syn::Error::new_spanned(expr, message);
    err.combine(other_error);
    err
}

/// Parse a tuple of (tag, string) where the tag must match the given tag.
fn parse_tuple_with_tagged_string(tag: &str, expr: &syn::Expr) -> syn::Result<String> {
    match parse_tuple_string_string(&expr) {
        Result::Ok((k, v)) => {
            if tag == k {
                syn::Result::Ok(v)
            } else {
                syn::Result::Err(syn::Error::new_spanned(expr, "Did not find expected tag."))
            }
        }
        Result::Err(inner) => syn::Result::Err(combined_error(
            expr,
            "not a tuple of (key,value) strings!",
            inner,
        )),
    }
}

/// Parse a documentation tuple, e.g.: ("documentation", [("label", "ID"), ("description", "The unique Bar entity ID.")])
fn parse_documentation_tuple(expr: &syn::Expr) -> syn::Result<HashMap<String, String>> {
    let docs_pair = parse_tuple_string_map(expr);
    let docs_map = match docs_pair {
        syn::Result::Ok(dp) => match get_named_value_from_pair("documentation", dp) {
            Some(dm) => {
                match (dm.contains_key("label"), dm.contains_key("description")) {
                    (true, true) => syn::Result::Ok(dm),
                    (false, true) => syn::Result::Err(syn::Error::new_spanned(expr, "Expected documentation map to include a \"label\" tuple: (\"label\", ...)")),
                    (true, false) => syn::Result::Err(syn::Error::new_spanned(expr, "Expected documentation map to include a \"description\" tuple: (\"description\", ...)")),
                    _ => syn::Result::Err(syn::Error::new_spanned(expr, "Expected documentation map to include a \"label\"  and a \"description\" tuple")),
                }
            }
            None => syn::Result::Err(syn::Error::new_spanned(expr, "Could not parse documentation tuple, first element of the tuple must be \"documentation\".")),
        },
        syn::Result::Err(e) => syn::Result::Err(combined_error(expr, "Invalid documentation tuple", e)),
    };
    docs_map
}

fn parse_tuple_with_string_and_array_of_array_of_name_and_documentation_and_type_tuples(
    expr: &syn::Expr,
) -> syn::Result<(String, Vec<(String, HashMap<String, String>, String)>)> {
    match expr {
        syn::Expr::Tuple(syn::ExprTuple {
            attrs: _,
            paren_token: _,
            elems,
        }) => {
            let mut i = elems.iter();
            match (i.next(), i.next(), i.next()) {
                (Some(tag_expr), Some(outer_array_expr), None) => {
                    println!("🚀🚀🚀 tag_expr = {:?}", tag_expr);
                    let opt_tag = parse_lit_string(tag_expr);
                    match opt_tag {
                        syn::Result::Ok(tag) => {
                            println!("🚀🚀🚀 outer_array_expr = {:?}", outer_array_expr);
                            match outer_array_expr {
                                syn::Expr::Array(ae) => match ae {
                                    syn::ExprArray {
                                        attrs: _,
                                        bracket_token: _,
                                        elems,
                                    } => {
                                        // the elems of the outer array are the inner arrays
                                        let mut result : Vec<(String, HashMap<String, String>, String)> = vec![];
                                        for inner_array_expr in elems.iter() {
                                            match inner_array_expr {
                                                syn::Expr::Array(iae) => match iae {
                                                    syn::ExprArray {
                                                        attrs: _,
                                                    bracket_token: _,
                                                    elems,
                                                    } => {
                                                        //println!("🚀🚀🚀 inner_array_expr = {:?}", iae);
                                                        let mut fi = elems.iter();
                                                        match (fi.next(), fi.next(), fi.next(), fi.next()) {
                                                            (Some(name_expr), Some(docs_expr), Some(type_expr), None) => {
                                                                let opt_name = parse_tuple_with_tagged_string("name", &name_expr);
                                                                println!("🚀🚀🚀 >>> opt_name_kv = {:?}", opt_name);
                                                                let opt_docs = parse_documentation_tuple(&docs_expr);
                                                                println!("🚀🚀🚀 >>> opt_docs = {:?}", opt_docs);
                                                                let opt_type = parse_tuple_with_tagged_string("type", &type_expr);
                                                                println!("🚀🚀🚀 >>> opt_type = {:?}", opt_type);

                                                                match (opt_name, opt_docs, opt_type) {
                                                                    // TODO: error reporting
                                                                    (syn::Result::Ok(name), syn::Result::Ok(docs), syn::Result::Ok(ty)) => {result.push((name, docs, ty));},
                                                                    _ => todo!("invalid name, docs, or type")
                                                                }
                                                            },
                                                            _ => todo!("expected a tuple with name, docs and type"),
                                                        }
                                                    },
                                                },
                                                _ => todo!("expected inner array element in outer array"),
                                            }
                                        }
                                        syn::Result::Ok((tag, result))
                                    }
                                },
                                _ => todo!("expected an array expression following the tag"),
                            }
                        },
                        syn::Result::Err(e) => syn::Result::Err(combined_error(tag_expr, "expected a literal string tag as first element of tuple: (tag, [name-doc-tuples...])", e))
                    }
                }
                _ => todo!("expected tuple of two elements: (tag, [name-doc-tuples...])"),
            }
        }
        _ => syn::Result::Err(syn::Error::new_spanned(expr, "Expected a Tuple expression")),
    }
}

/// Create a metamodel Name object from a string
fn to_metamodel_name(name: &str) -> metamodel::Name {
    metamodel::Name::Literal(name.to_string())
}

/// Create the metamodel Documentation object from a map of key-values
fn to_metamodel_documentation(dm: &HashMap<String, String>) -> metamodel::Documentation {
    metamodel::Documentation::new(dm.get("label").unwrap(), dm.get("description").unwrap())
}

// TODO: use Into trait

/// Create a metamodel Type object from a string
fn to_metamodel_type(type_name: &str) -> metamodel::Type {
    match type_name {
        "ID" => metamodel::Type::Primitive(metamodel::PrimitiveType::Id),
        "LocalDate" => metamodel::Type::Primitive(metamodel::PrimitiveType::LocalDate),
        "String" => metamodel::Type::Primitive(metamodel::PrimitiveType::String),
        _ => todo!("unknown type {:?}", type_name),
    }
}

/// Parse the tuple language representation of the meta-model
pub fn parse_tuple_expression_to_metamodel(input: TokenStream) -> Result<metamodel::Expr> {
    let item: syn::Expr = syn::parse(input).expect("failed to parse input");
    let ast: Result<metamodel::Expr> = match item {
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
                        syn::Result::Ok(tag) => match tag.as_str() {
                            "record" => {
                                println!("🚀🚀🚀 compiling a RECORD type!");
                                match (expr1, expr2, expr3) {
                                    (Some(name_expr),Some(fields_expr), None) => {
                                        match parse_array_with_name_and_documentation_tuple(&name_expr) {
                                            Result::Ok((n, dm)) => {
                                                let opt_fields = parse_tuple_with_string_and_array_of_array_of_name_and_documentation_and_type_tuples(&fields_expr);

                                                println!("🚀🚀🚀 compiling a RECORD type with fields: {:?}", opt_fields);
                                                let fields = match opt_fields {
                                                    syn::Result::Ok((tag, field_name_docs)) => {
                                                        match tag.as_str() {
                                                            "fields" => {
                                                                println!("🚀🚀🚀 compiling fields: {:?}", field_name_docs);
                                                                let mut fields : Vec<metamodel::FieldDeclaration> = vec![];
                                                                for (name, docs, ty) in field_name_docs.iter() {
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

                                                Result::Ok(metamodel::Expr::RecordDeclarationExpr(
                                                        metamodel::RecordDeclaration::new(
                                                                to_metamodel_name(&n),
                                                        to_metamodel_documentation(&dm),
                                                        fields,
                                                        ),
                                                ))
                                            },
                                            Result::Err(e) => {
                                                // todo!("could not parse name and documentation tuple"),
                                                //let mut err = syn::Error::new_spanned(item, "could not parse name and documentation tuple");
                                                //&err.combine(_e);
                                                syn::Result::Err(e)
                                            }

                                        }
                                    },
                                    (Some(_),None, None) => todo!("invalid record declaration (fields missing), expected tuple of three elements, (tag, [name...], (fields [...]))"),
                                    _ => todo!("invalid record declaration, expected tuple of three elements, (tag, [name...], (fields [...]))"),
                                }
                                // end "record"
                            }
                            _ => todo!("unknown tag"),
                        },
                        syn::Result::Err(_e) => {
                            todo!("No literal string tag found as first element of first tuple.")
                        }
                    }
                }
                (None, _, _, _) => todo!("Expected non-empty tuple"),
            }
        }
        _ => todo!("expected a tuple"),
    };
    ast
}


#[cfg(test)]
mod playground_tests {
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
