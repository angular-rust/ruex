use std::{error, fmt, result};

use proc_macro::TokenStream;
use proc_macro2::TokenTree;
use syn::*;

type Result<T> = result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, PartialEq)]
enum DecoratorError {
    InvaludTokenStream,
    DecoratorNotFound,
}

impl std::fmt::Display for DecoratorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DecoratorError::InvaludTokenStream => write!(f, "Invalid token stream"),
            DecoratorError::DecoratorNotFound => write!(f, "Decorator name not found"),
        }
    }
}

impl error::Error for DecoratorError {}

#[derive(Debug, PartialEq)]
enum DecoratorAttr {
    Fixed { name: Ident },
    Parametric { name: Ident, args: Vec<Expr> },
}

impl DecoratorAttr {
    fn parse(attr: proc_macro2::TokenStream) -> Result<Self> {
        let mut ident = None;
        let mut args = Vec::new();
        for at in attr {
            match at {
                TokenTree::Ident(id) => {
                    ident = Some(id);
                }
                TokenTree::Group(grp) => {
                    if ident.is_none() {
                        return Err(DecoratorError::InvaludTokenStream)?;
                    }
                    for t in grp.stream() {
                        if let Ok(expr) = syn::parse2(t.into()) {
                            args.push(expr);
                        }
                    }
                }
                _ => return Err(DecoratorError::InvaludTokenStream)?,
            }
        }
        if let Some(name) = ident {
            if args.is_empty() {
                Ok(DecoratorAttr::Fixed { name })
            } else {
                Ok(DecoratorAttr::Parametric { name, args })
            }
        } else {
            return Err(DecoratorError::DecoratorNotFound)?;
        }
    }
}

pub(crate) fn decorate(attr: TokenStream, func: TokenStream) -> TokenStream {
    let func = func.into();
    let item_fn: ItemFn = syn::parse(func).expect("Input is not a function");
    let vis = &item_fn.vis;
    let ident = &item_fn.sig.ident;
    let block = &item_fn.block;

    let inputs = item_fn.sig.inputs;
    let output = item_fn.sig.output;

    let input_values: Vec<_> = inputs
        .iter()
        .map(|arg| match arg {
            &FnArg::Typed(ref val) => &val.pat,
            _ => unimplemented!("#[decorate] cannot be used with associated function"),
        })
        .collect();

    let attr = DecoratorAttr::parse(attr.into()).expect("Failed to parse attribute");
    let caller = match attr {
        DecoratorAttr::Fixed { name } => {
            quote::quote! {
                #vis fn #ident(#inputs) #output {
                    let f = #name(deco_internal);
                    return f(#(#input_values,) *);

                    fn deco_internal(#inputs) #output #block
                }
            }
        }
        DecoratorAttr::Parametric { name, args } => {
            quote::quote! {
                #vis fn #ident(#inputs) #output {
                    let deco = #name(#(#args,) *);
                    let f = deco(deco_internal);
                    return f(#(#input_values,) *);

                    fn deco_internal(#inputs) #output #block
                }
            }
        }
    };
    caller.into()
}