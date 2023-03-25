use std::{
    collections::{HashMap, VecDeque},
    net::UdpSocket,
};

use companion::{companion_addr, Response, Task};
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{quote, ToTokens};
use syn::{
    visit_mut::VisitMut, Attribute, FnArg, Ident, Item, ItemTrait, Pat, Path, TraitItem,
    TraitItemMethod,
};

use crate::MAX_UDP_PAYLOAD;

use super::{AspectJointPoint, ContractAspectState, Enum, SyntaxAndDocs, Type};

/// Name used for the "re-routed" method.
fn trait_method_impl_name(name: &str) -> String {
    format!("__impl_trait_{}", name)
}

fn method_rename(method: &TraitItemMethod) -> TraitItemMethod {
    let mut method: TraitItemMethod = (*method).clone();
    let name = trait_method_impl_name(&method.sig.ident.to_string());

    let mut attrs = vec![];
    attrs.push(syn::parse_quote!(#[doc(hidden)]));
    attrs.push(syn::parse_quote!(#[doc = " This is an internal function that is not meant to be used directly!"]));
    attrs
        .push(syn::parse_quote!(#[doc = " See the documentation of the `#[register]` attribute."]));

    // add all existing non-contract attributes
    attrs.extend(
        method
            .attrs
            .iter()
            .filter(|attr| {
                let name = attr.path.segments.last().unwrap().ident.to_string();

                Type::type_and_mode(&name).is_none() && name != "aspect" && name != "doc"
            })
            .cloned(),
    );

    method.attrs = attrs;
    method.sig.ident = Ident::new(&name, method.sig.ident.span());

    method
}

struct ArgInfo {
    call_toks: proc_macro2::TokenStream,
}

// Calculate name and pattern tokens
fn arg_pat_info(pat: &Pat) -> ArgInfo {
    match pat {
        Pat::Ident(ident) => {
            let toks = quote::quote! {
                #ident
            };
            ArgInfo { call_toks: toks }
        }
        Pat::Tuple(tup) => {
            let infos = tup.elems.iter().map(arg_pat_info);

            let toks = {
                let mut toks = proc_macro2::TokenStream::new();

                for info in infos {
                    toks.extend(info.call_toks);
                    toks.extend(quote::quote!(,));
                }

                toks
            };

            ArgInfo {
                call_toks: quote::quote!((#toks)),
            }
        }
        Pat::TupleStruct(_tup) => unimplemented!(),
        p => panic!("Unsupported pattern type: {:?}", p),
    }
}

#[allow(unused_variables)]
fn process_item_trait(path: String, mut input: ItemTrait) -> TokenStream2 {
    // create method wrappers and renamed items
    let funcs = input
        .items
        .iter()
        .filter_map(|item| {
            if let TraitItem::Method(method) = item {
                let rename = method_rename(method);
                let wrapper = {
                    // create method wrapper
                    let mut method = (*method).clone();
                    let args = method
                        .sig
                        .inputs
                        .clone()
                        .into_iter()
                        .map(|arg| {
                            //
                            match &arg {
                                FnArg::Receiver(_) => quote! {self},
                                FnArg::Typed(p) => {
                                    let info = arg_pat_info(&p.pat);

                                    info.call_toks
                                }
                            }
                        })
                        .collect::<Vec<_>>();
                    let arguments = {
                        let mut toks = proc_macro2::TokenStream::new();

                        for arg in args {
                            toks.extend(arg);
                            toks.extend(quote::quote!(,));
                        }

                        toks
                    };

                    // pre-process here
                    let body: TokenStream2 = {
                        let name = trait_method_impl_name(&method.sig.ident.to_string());
                        let name = syn::Ident::new(&name, method.sig.ident.span());

                        quote::quote! {
                            {
                                Self::#name(#arguments)
                            }
                        }
                    };

                    let mut attrs = vec![];

                    // keep the documentation and contracts of the original method
                    attrs.extend(
                        method
                            .attrs
                            .iter()
                            .filter(|attr| {
                                let name = attr.path.segments.last().unwrap().ident.to_string();
                                Type::type_and_mode(&name).is_some()
                                    || name == "aspect"
                                    || name == "doc"
                            })
                            .cloned(),
                    );
                    // always inline
                    attrs.push(syn::parse_quote!(#[inline(always)]));

                    // INFO: NEW PROCESS
                    let mut state: ContractAspectState = ContractAspectState::default();
                    let attrs = state.process(&attrs);

                    let variables = state.variables();

                    let mut contract_docs: VecDeque<String> = VecDeque::new();

                    let (requires, docs) = state.requires.generate();
                    contract_docs.extend(docs);

                    let (invariants, docs) = state.invariants.generate();
                    contract_docs.extend(docs);

                    let (ensures, docs) = state.ensures.generate();
                    contract_docs.extend(docs);

                    if !contract_docs.is_empty() {
                        contract_docs.push_front(String::from(" # Contract"));
                    }

                    let before = state
                        .aspects
                        .iter()
                        .filter_map(|aspect| {
                            if let Some(item) = &aspect.before {
                                item.default.as_ref().map(|block| {
                                    let stmts = &block.stmts;
                                    quote! {
                                        #(#stmts)*
                                    }
                                })
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>();

                    let after = state
                        .aspects
                        .iter()
                        .rev()
                        .filter_map(|aspect| {
                            if let Some(item) = &aspect.after {
                                item.default.as_ref().map(|block| {
                                    let stmts = &block.stmts;
                                    quote! {
                                        #(#stmts)*
                                    }
                                })
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>();

                    // Here we should deal with call name )))
                    // For traits we need only change some in output
                    // let mut around = quote! {inner()}; // body
                    let mut around = body.clone(); // body
                    let callee = body.clone();
                    let mut has_around = false;

                    let mut it = state.aspects.iter().rev().peekable();
                    while let Some(aspect) = it.next() {
                        if let Some(item) = &aspect.around {
                            item.default.as_ref().map(|block| {
                                let mut replacer = AspectJointPoint { stream: &around };

                                let mut stmts = block.stmts.clone();

                                for stmt in stmts.iter_mut() {
                                    replacer.visit_stmt_mut(stmt);
                                }

                                if it.peek().is_none() {
                                    around = quote!(#(#stmts)*);
                                } else {
                                    around = quote!({#(#stmts)*});
                                }
                                has_around = true;
                            });
                        }
                    }

                    let attrs = {
                        let mut new_attrs: Vec<Attribute> = Vec::new();

                        if !state.docs.is_empty() {
                            let mut it = state.docs.iter().peekable();
                            while let Some(comment) = it.next() {
                                if it.peek().is_some() {
                                    new_attrs.push(syn::parse_quote!(#[doc = #comment]));
                                } else if !comment.trim().is_empty() {
                                    new_attrs.push(syn::parse_quote!(#[doc = #comment]));
                                }
                            }
                            new_attrs.push(syn::parse_quote!(#[doc = ""]));
                        }

                        if !contract_docs.is_empty() {
                            let mut it = contract_docs.iter().peekable();
                            while let Some(comment) = it.next() {
                                if it.peek().is_some() {
                                    new_attrs.push(syn::parse_quote!(#[doc = #comment]));
                                } else if !comment.trim().is_empty() {
                                    new_attrs.push(syn::parse_quote!(#[doc = #comment]));
                                }
                            }
                            new_attrs.push(syn::parse_quote!(#[doc = ""]));
                        }

                        let mut aspect_docs: VecDeque<String> = VecDeque::new();
                        // INFO: Aspects
                        state.aspects.iter().for_each(|aspect| {
                            //
                            let mut docs = aspect.documentation();
                            if !docs.is_empty() {
                                docs.push_back(String::new());
                                aspect_docs.extend(docs);
                            }
                        });

                        if !aspect_docs.is_empty() {
                            aspect_docs.push_front(String::from(" # Aspects"));
                            let mut it = aspect_docs.iter().peekable();
                            while let Some(comment) = it.next() {
                                if it.peek().is_some() {
                                    new_attrs.push(syn::parse_quote!(#[doc = #comment]));
                                } else if !comment.trim().is_empty() {
                                    new_attrs.push(syn::parse_quote!(#[doc = #comment]));
                                }
                            }
                            new_attrs.push(syn::parse_quote!(#[doc = ""]));
                        }
                        attrs.iter().cloned().for_each(|attr| {
                            new_attrs.push(attr);
                        });
                        new_attrs
                    };

                    // INFO: END OF NEW PROCESS
                    method.attrs = attrs;

                    {
                        let result = if has_around {
                            quote! {
                                let result = #callee;
                            }
                        } else {
                            quote! {
                                let result = {
                                    #around
                                };
                            }
                        };
                        let body = quote! {
                            {
                                #(#requires)*
                                #(#invariants)*

                                #(#variables)*

                                #(#before)*

                                #result

                                #(#after)*

                                #(#invariants)*
                                #(#ensures)*

                                result
                            }
                        };
                        let block: syn::Block = syn::parse2(body).unwrap();
                        method.default = Some(block);
                        method.semi_token = None;
                    }

                    method
                };

                Some(vec![TraitItem::Method(rename), TraitItem::Method(wrapper)])
            } else {
                None
            }
        })
        .flatten()
        .collect::<Vec<_>>();

    // remove all previous methods
    input.items = input
        .items
        .into_iter()
        .filter(|item| {
            //
            match item {
                TraitItem::Method(_) => false,
                _ => true,
            }
        })
        .collect();

    // add back new methods
    input.items.extend(funcs);

    let _ = process_remote_trait(path, input.clone());

    input.into_token_stream()
}

// Remote processing
// Cutoff all attributes
fn process_remote_trait(path: String, mut input: ItemTrait) -> TokenStream2 {
    // Cutoff remote attributes
    input.attrs = vec![];
    input.items.iter_mut().for_each(|item| {
        // Cutoff every remote methods attributes and body
        if let TraitItem::Method(method) = item {
            method.attrs = vec![];

            if method.default.is_some() {
                method.default = None;
                method.semi_token = Some(syn::token::Semi(Span::call_site()));
            }
        }
    });
    let data = syn_serde::json::to_string(&input);

    let mut buf = [0; MAX_UDP_PAYLOAD];

    let addr = companion_addr();

    let socket = UdpSocket::bind("[::]:0").unwrap();
    socket.connect(addr).unwrap();

    socket.send(&Task::Set(&path, &data).as_bytes()).unwrap();
    let (len, _src) = socket.recv_from(&mut buf).unwrap();
    let _resp = Response::from(&buf[..len]);

    TokenStream2::new()
}

fn process_enum_trait(path: String, input: ItemTrait) -> TokenStream2 {
    let supertraits = input
        .supertraits
        .iter()
        .filter_map(|item| match item {
            syn::TypeParamBound::Trait(item) => {
                //
                Some(
                    item.path
                        .segments
                        .iter()
                        .map(|segment| segment.ident.to_string())
                        .collect::<Vec<_>>()
                        .join("::"),
                )
            }
            syn::TypeParamBound::Lifetime(_) => None,
        })
        .collect::<Vec<_>>();

    let item = Enum {
        supertraits,
        items: HashMap::new(),
    };

    let data = serde_json::to_string(&item).unwrap();

    let mut buf = [0; MAX_UDP_PAYLOAD];

    let addr = companion_addr();

    let socket = UdpSocket::bind("[::]:0").unwrap();
    socket.connect(addr).unwrap();

    socket.send(&Task::Set(&path, &data).as_bytes()).unwrap();
    let (len, _src) = socket.recv_from(&mut buf).unwrap();
    let _resp = Response::from(&buf[..len]);

    TokenStream2::new()
}

fn process_aspect_trait(path: String, input: ItemTrait) -> TokenStream2 {
    input.items.iter().for_each(|item| {
        // check input
        match item {
            TraitItem::Method(method) => {
                let ident = method.sig.ident.to_string();
                match ident.as_str() {
                    "before" | "after" | "around" => {}
                    _ => {
                        panic!("Aspect supports only `before`, `after` and `around` methods")
                    }
                }
            }
            _ => panic!("Aspect definition support only methods"),
        }
    });
    let data = syn_serde::json::to_string(&input);

    let mut buf = [0; MAX_UDP_PAYLOAD];

    let addr = companion_addr();

    let socket = UdpSocket::bind("[::]:0").unwrap();
    socket.connect(addr).unwrap();

    socket.send(&Task::Set(&path, &data).as_bytes()).unwrap();
    let (len, _src) = socket.recv_from(&mut buf).unwrap();
    let _resp = Response::from(&buf[..len]);

    input.into_token_stream()
}

pub(crate) fn register(attrs: TokenStream2, input: TokenStream2) -> TokenStream2 {
    let path: Path = syn::parse2(attrs).unwrap();
    let path = path
        .segments
        .iter()
        .map(|item| item.ident.to_string())
        .collect::<Vec<String>>()
        .join("::");

    let item: Item = syn::parse2(input).unwrap();
    match item {
        Item::Trait(item) => {
            //
            let ident = item.ident.to_string();
            if ident.ends_with("Aspect") {
                process_aspect_trait(path, item)
            } else if ident.ends_with("Remote") {
                process_remote_trait(path, item)
            } else if ident.ends_with("Enum") {
                process_enum_trait(path, item)
            } else {
                process_item_trait(path, item)
            }
        }
        _ => panic!("Traits only supported for registration"),
    }
}
