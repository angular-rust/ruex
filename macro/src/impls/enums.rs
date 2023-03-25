#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
// necessary for the TokenStream::from_str() implementation
use std::{collections::HashMap, net::UdpSocket, path::PathBuf, str::FromStr};

use companion::{companion_addr, Response, Task};
// use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use serde::{Deserialize, Serialize, Serializer};
use serde_tokenstream::from_tokenstream;
use syn::{Item, ItemFn, ItemStruct, Path};

use crate::MAX_UDP_PAYLOAD;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ItemLocation {
    pub path: String,
    pub range: (usize, usize),
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Enum {
    pub supertraits: Vec<String>,
    pub items: HashMap<String, ItemLocation>,
}

pub(crate) fn mount(
    attrs: TokenStream2,
    input: TokenStream2,
    location: ItemLocation,
) -> TokenStream2 {
    let path: Path = syn::parse2(attrs).unwrap();
    let path = path
        .segments
        .iter()
        .map(|item| item.ident.to_string())
        .collect::<Vec<String>>()
        .join("::");
    let item: Item = syn::parse2(input.clone()).unwrap();
    match item {
        Item::Struct(item) => {
            let ident = item.ident.to_string();
            // println!("Place {ident} into {path}");
            let mut buf = [0; MAX_UDP_PAYLOAD];

            let addr = companion_addr();

            let socket = UdpSocket::bind("[::]:0").unwrap();
            socket.connect(addr).unwrap();

            socket.send(&Task::Get(&path).as_bytes()).unwrap();
            let (len, _src) = socket.recv_from(&mut buf).unwrap();
            let resp = Response::from(&buf[..len]);

            if let Response::String(data) = resp {
                // println!("{data}");
                let mut data: Enum = serde_json::from_str(&data).unwrap();
                data.items.insert(ident, location);
                // println!("{data:#?}");

                let data = serde_json::to_string(&data).unwrap();
                // println!("{data}");
                socket.send(&Task::Set(&path, &data).as_bytes()).unwrap();
                let (len, _src) = socket.recv_from(&mut buf).unwrap();
                let _resp = Response::from(&buf[..len]);
            } else {
                panic!("Enum Trait `{}` is not registered", path)
            }
        }
        _ => panic!("Struct only supported for mounting"),
    }
    input
}
