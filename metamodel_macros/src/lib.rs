extern crate metamodel;

use proc_macro::{self, TokenStream};
use quote::quote;
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
    println!("🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀 running macro...");
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
                        }) => {
                            Some(ident.clone())
                        }
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
    } }.into()
}
