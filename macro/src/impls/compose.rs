use std::net::UdpSocket;

// use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream, Result},
    punctuated::Punctuated,
    Data, DeriveInput, Field, Fields, Ident, Path, Token, TraitItem,
};

use companion::{companion_addr, Response, Task};

use crate::MAX_UDP_PAYLOAD;

#[derive(Default, Debug)]
struct DelegateArgs {
    paths: Vec<Path>,
}

impl Parse for DelegateArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let items;
        syn::parenthesized!(items in input);

        let types: Punctuated<Path, Token![,]> = items.parse_terminated(syn::Path::parse).unwrap();
        let paths = types.into_iter().collect::<Vec<_>>();

        Ok(DelegateArgs { paths })
    }
}

struct DelegateVar {
    var: Ident,
    args: Vec<DelegateArgs>,
}

fn filter_fields(field: &Field) -> Option<DelegateVar> {
    if let Some(ident) = &field.ident {
        let args: Vec<DelegateArgs> = field
            .attrs
            .iter()
            .filter_map(|attr| {
                let init = String::new();
                let attribute = attr.path.segments.iter().fold(init, |acc, item| {
                    if acc.is_empty() {
                        format!("{}", item.ident)
                    } else {
                        format!("{acc}::{}", item.ident)
                    }
                });

                if attribute == "delegate" {
                    match syn::parse2::<DelegateArgs>(attr.tokens.clone()) {
                        Ok(path) => Some(path),
                        Err(_) => {
                            panic!("usage: #[delegate(std::fmt::Display, Debug)]");
                        }
                    }
                } else {
                    None
                }
            })
            .collect();
        Some(DelegateVar {
            var: ident.clone(),
            args,
        })
    } else {
        None
    }
}

fn collect(data: &Data) -> Vec<DelegateVar> {
    match &data {
        Data::Struct(datastruct) => match &datastruct.fields {
            Fields::Named(fields) => {
                let collected: Vec<DelegateVar> =
                    fields.named.iter().filter_map(filter_fields).collect();
                collected
            }
            Fields::Unnamed(_) => todo!(),
            Fields::Unit => todo!(),
        },
        Data::Enum(_) => todo!(),
        Data::Union(_) => todo!(),
    }
}

fn generate(var: &Ident, items: &Vec<TraitItem>) -> Vec<TokenStream2> {
    items
        .iter()
        .filter_map(|item| {
            if let TraitItem::Method(method) = item {
                Some(&method.sig)
            } else {
                None
            }
        })
        .map(|signature| {
            let syn::Signature {
                ident: sig, inputs, ..
            } = &signature;
            let inputs = inputs
                .iter()
                .filter_map(|arg| match arg {
                    syn::FnArg::Receiver(_) => None,
                    syn::FnArg::Typed(val) => {
                        if let syn::Pat::Ident(pat) = val.pat.as_ref() {
                            Some(pat.ident.clone())
                        } else {
                            None
                        }
                    }
                })
                .collect::<Vec<Ident>>();

            quote! {
                #signature {
                    self.#var.#sig(#(#inputs), *)
                }
            }
        })
        .collect::<Vec<TokenStream2>>()
}

pub(crate) fn compose(item: TokenStream2) -> TokenStream2 {
    let input: DeriveInput = syn::parse2(item.clone()).unwrap();

    let DeriveInput { ident, data, .. } = input;

    let collected = collect(&data);

    let addr = companion_addr();

    let socket = UdpSocket::bind("[::]:0").unwrap();
    socket.connect(addr).unwrap();
    let mut buf = [0; MAX_UDP_PAYLOAD];

    let impls: Vec<TokenStream2> = collected
        .iter()
        .map(|item| {
            let var = &item.var;
            item.args
                .iter()
                .map(|item| {
                    item.paths
                        .iter()
                        .map(|path| {
                            let str_path = path
                                .segments
                                .iter()
                                .map(|item| item.ident.to_string())
                                .collect::<Vec<String>>()
                                .join("::");

                            socket.send(&Task::Get(&str_path).as_bytes()).unwrap();
                            let (len, _src) = socket.recv_from(&mut buf).unwrap();
                            let resp = Response::from(&buf[..len]);

                            if let Response::String(data) = resp {
                                let def: syn::ItemTrait = syn_serde::json::from_str(&data).unwrap();

                                let syn::ItemTrait { items, .. } = def;
                                let methods = generate(&var, &items);

                                quote! {
                                    impl #path for #ident {
                                        #(#methods)*
                                    }
                                }
                            } else {
                                panic!("Trait `{}` is not registered", str_path)
                            }
                        })
                        .collect::<Vec<TokenStream2>>()
                })
                .flatten()
                .collect::<Vec<TokenStream2>>()
        })
        .flatten()
        .collect();
    quote! {
        #(#impls)*
    }
}
