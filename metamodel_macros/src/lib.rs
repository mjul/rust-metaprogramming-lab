extern crate metamodel;

use proc_macro::{self, TokenStream};
use quote::quote;
use syn::parse::Parse;
use syn::{Result, Token};

mod codegen;
mod tuple_lang;

#[derive(Debug)]
struct GenStructsInputEnum {}

impl Parse for GenStructsInputEnum {
    fn parse(_input: syn::parse::ParseStream) -> Result<Self> {
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

    let _model_identifier = match item {
        syn::Expr::Call(c) => match c {
            syn::ExprCall {
                attrs: _,
                func,
                args: _,
                paren_token: _,
            } => match *func {
                syn::Expr::Path(p) => match p {
                    syn::ExprPath {
                        attrs: _,
                        qself: _,
                        path: _,
                    } => {
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
    quote!(
    struct Foo { id: usize};
    impl Foo { pub fn new(id:usize) -> Self {
        Self { id }
    }
    }
    )
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

#[proc_macro]
pub fn eval(input: TokenStream) -> TokenStream {
    input.into()
}

#[proc_macro]
pub fn generate_model_from_expression_stream(input: TokenStream) -> TokenStream {
    println!("🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀 running macro: generate_model_from_expression_stream...");

    println!("🔍 parsing tuple expression...");
    let mut result: TokenStream = TokenStream::new();
    result.extend::<TokenStream>("let model : metamodel::Expr =".parse().unwrap());
    result.extend(input);
    result.extend::<TokenStream>(";".parse().unwrap());
    result.extend::<TokenStream>("extern crate metamodel_macros;".parse().unwrap());
    //result.extend::<TokenStream>("use metamodel_macros::codegen::{generate_codex_for_meta_model};".parse().unwrap());
    result.extend::<TokenStream>("metamodel_macros::eval!(metamodel_macros::codegen::generate_code_for_meta_model(model).into()).into()".parse().unwrap());

    println!("📝 generating code...");
    println!("🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀 macro completed.");
    result.into()
}

/*

// these proc_macros cause the failure (the ignore macro called from the this_fails macro)
// this code is in root of a crate named metamodel_macros

#[proc_macro]
pub fn ignore(input: TokenStream) -> TokenStream {
    let mut result : TokenStream = TokenStream::new();
    result.into()
}

#[proc_macro]
pub fn this_fails(input: TokenStream) -> TokenStream {
    let mut result : TokenStream = TokenStream::new();
    result.extend::<TokenStream>("metamodel_macros::ignore!(42).into()".parse().unwrap());
    result.into()
}

*/
