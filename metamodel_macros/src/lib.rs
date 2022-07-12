extern crate metamodel;

use proc_macro::{self, TokenStream};
use quote::quote;
use std::collections::HashMap;
use std::fmt;
use syn::parse::Parse;
use syn::{parse_macro_input, Result, Token};

mod codegen;
mod tuple_lang;

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
                _ => todo!("generate_data_structures: Error matching function call, not a Path"),
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
                        None => todo!("x1"),
                        _ => todo!("x2"),
                    },
                    _ => todo!("x3"),
                },
                _ => todo!("x4"),
            },
            _ => todo!("x4"),
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

#[proc_macro]
pub fn generate_model_from_tuple(input: TokenStream) -> TokenStream {
    println!("🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀 running macro: generate_model_from_tuple...");
    println!("🔍 parsing tuple expression...");
    let ast = tuple_lang::parse_tuple_expression_to_metamodel(input);
    println!("📝 generating code...");
    let result = match ast {
        Result::Ok(r) => codegen::generate_code_for_meta_model(r),
        Err(e) => e.to_compile_error().into(),
    };
    println!("🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀 macro completed.");
    result
}
