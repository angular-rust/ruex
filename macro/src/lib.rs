#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(non_snake_case)]

use proc_macro::TokenStream;
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, DeriveInput, ItemFn, Ident, Token};
use quote::quote;

struct AdviceField {
    member: Ident,
    // colon: Token![:],
    value: syn::LitStr,
}

impl Parse for AdviceField {
    fn parse(input: ParseStream) -> Result<Self> {
        let member: Ident = input.parse()?;
        let colon_token: Token![:] = input.parse()?;
        // let value: Expr = input.parse()?;
        let value: syn::LitStr = input.parse()?;

        Ok(AdviceField {
            member,
            // colon_token,
            value,
        })
    }
}

struct Args {
    // advice: Ident,
    // before: Ident,
    // after: Ident,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> Result<Self> {
        let vars = Punctuated::<AdviceField, Token![,]>::parse_terminated(input)?;
        let idents: Vec<AdviceField> = vars.into_iter().collect();
        // dbg!(idents);
        // todo!("GOOD")
        Ok(Args {
            // vars: vars.into_iter().collect(),
        })
    }
}


#[proc_macro_attribute]
pub fn Aspect(attr: TokenStream, item: TokenStream) -> TokenStream {
    // println!("attr: \"{}\"", attr.to_string());
    let attr = attr.clone();
    let attr = syn::parse_macro_input!(attr as Args);

    // println!("item: \"{}\"", item.to_string());

    item
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
