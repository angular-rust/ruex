use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

use quote::{format_ident, quote};
use syn::{FnArg, Item, ItemTrait, ReturnType, TraitItem, TypeParamBound};

fn process_item_trait(mut input: ItemTrait) -> TokenStream2 {
    let trait_name = input.ident.clone();
    let mock_name = format_ident!("Mock{}", trait_name);
    let builder_name = format_ident!("Mock{}Builder", trait_name);

    // Fix default supertrait
    {
        let mut has_default = false;
        input.supertraits.iter().for_each(|item| {
            //
            if let TypeParamBound::Trait(val) = item {
                let name = val.path.segments.last().unwrap().ident.to_string();
                if &name == "Default" {
                    has_default = true;
                }
            }
        });
        if !has_default {
            let value: syn::TraitBound = syn::parse_quote! {Default};
            input.supertraits.insert(0, TypeParamBound::Trait(value));
        }
    }

    // Collect method names and generate mock method
    let fields = input
        .items
        .iter()
        .filter_map(|item| {
            if let TraitItem::Method(method) = item {
                Some(format_ident!("__mock_{}__", method.sig.ident))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    let mock_method: syn::TraitItemMethod = syn::parse_quote! {
        fn mock() -> #builder_name<Self> {
            #builder_name {
                inner: #mock_name {
                    __mock_fallback__: Default::default(),
                    #(#fields: None,)*
                }
            }
        }
    };

    let mut builder_methods: Vec<TokenStream2> = vec![];
    // Generate mock struct
    let mock = {
        let fields = input
            .items
            .iter()
            .filter_map(|item| {
                if let TraitItem::Method(method) = item {
                    let inputs = method
                        .sig
                        .inputs
                        .iter()
                        .filter_map(|input| {
                            if let FnArg::Typed(typed) = input {
                                Some(typed.ty.clone())
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>();
                    let field = format_ident!("__mock_{}__", method.sig.ident);
                    match method.sig.output {
                        ReturnType::Default => {
                            let builder_method = {
                                let method_name = &method.sig.ident;
                                quote! {
                                    fn #method_name<F>(mut self, when: F) -> Self
                                    where
                                        F: Fn(#(#inputs),*) + 'static,
                                    {
                                        self.inner.#field = Some(Box::new(when));
                                        self
                                    }
                                }
                            };
                            builder_methods.push(builder_method);
                            Some(quote! {#field: Option<Box<dyn Fn(#(#inputs),*)>>})
                        }
                        ReturnType::Type(_, ref ty) => {
                            let output = ty.clone();
                            let builder_method = {
                                let method_name = &method.sig.ident;
                                quote! {
                                    fn #method_name<F>(mut self, when: F) -> Self
                                    where
                                        F: Fn(#(#inputs),*) -> #output + 'static,
                                    {
                                        self.inner.#field = Some(Box::new(when));
                                        self
                                    }
                                }
                            };
                            builder_methods.push(builder_method);
                            Some(quote! {#field: Option<Box<dyn Fn(#(#inputs),*) -> #output>>})
                        }
                    }
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        quote! {
            #[derive(Default)]
            struct #mock_name<T>
            where
                T: #trait_name,
            {
                __mock_fallback__: T,
                #(#fields),*
            }
        }
    };

    let mock_impl = {
        let methods = input
            .items
            .iter()
            .filter_map(|item| {
                if let TraitItem::Method(method) = item {
                    let sig = method.sig.clone();
                    let args = sig
                        .inputs
                        .iter()
                        .filter_map(|input| {
                            if let FnArg::Typed(syn::PatType { pat, .. }) = input {
                                if let syn::Pat::Ident(ident) = pat.as_ref() {
                                    Some(&ident.ident)
                                } else {
                                    panic!("Unhandled function argument");
                                }
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>();
                    let ident = &sig.ident;
                    let field = format_ident!("__mock_{}__", sig.ident);
                    Some(quote! {
                        #sig {
                            match self.#field {
                                Some(ref func) => func(#(#args),*),
                                None => self.__mock_fallback__.#ident(#(#args),*),
                            }
                        }
                    })
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        quote! {
            impl<T> #trait_name for #mock_name<T>
            where
                T: #trait_name,
            {
                #(#methods)*
            }
        }
    };

    let builder = quote! {
        struct #builder_name<T>
        where
            T: #trait_name,
        {
            inner: #mock_name<T>,
        }
    };

    let builder_impl = quote! {
        impl<T> #builder_name<T>
        where
            T: #trait_name,
        {
            #(#builder_methods)*

            fn build(self) -> #mock_name<T> {
                self.inner
            }
        }
    };
    // output
    input.items.push(TraitItem::Method(mock_method));
    quote! {
        #input

        #mock

        #mock_impl

        #builder

        #builder_impl
    }
}

pub(crate) fn mock(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let item: Item = syn::parse2(input.into()).unwrap();
    match item {
        Item::Trait(item) => process_item_trait(item).into(),
        _ => panic!("Traits only supported for mocking"),
    }
}