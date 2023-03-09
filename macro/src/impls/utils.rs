#![allow(dead_code, unused_variables)]
use syn::{
    parse::{Parse, ParseStream, Result},
    punctuated::Punctuated,
    Ident, Token, TypePath,
};

pub(crate) const MAX_UDP_PAYLOAD: usize = 65507;

pub struct AdviceField {
    member: Ident,
    // colon: Token![:],
    // value: syn::LitStr,
    value: TypePath,
}

impl Parse for AdviceField {
    fn parse(input: ParseStream) -> Result<Self> {
        let member: Ident = input.parse()?;
        let colon_token: Token![:] = input.parse()?;
        // let value: Expr = input.parse()?;
        // let value: syn::LitStr = input.parse()?;
        let value: TypePath = input.parse()?;

        Ok(AdviceField {
            member,
            // colon_token,
            value,
        })
    }
}

pub struct Args {
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
