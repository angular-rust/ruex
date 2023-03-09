use proc_macro2::{Group, Span, TokenStream, TokenTree};
use quote::{quote, quote_spanned, ToTokens};
use syn::spanned::Spanned;
use syn::{
    parse_macro_input, Expr, ExprParen, FnArg, ImplItem, ImplItemMethod, ItemImpl, Pat, ReturnType,
};

fn delegate_input(input: TokenStream, receiver: &Expr) -> TokenStream {
    if let Ok(input) = syn::parse2::<ItemImpl>(input.clone()) {
        return delegate_impl_block(input, receiver);
    }
    if let Ok(input) = syn::parse2::<ImplItemMethod>(input.clone()) {
        return delegate_method(input, receiver);
    }
    let mut tokens = input.into_iter();
    let first_non_attr_token = 'outer: loop {
        match tokens.next() {
            None => break None,
            Some(TokenTree::Punct(p)) if p.as_char() == '#' => {}
            Some(token) => break Some(token),
        }
        loop {
            match tokens.next() {
                None => break 'outer None,
                Some(TokenTree::Punct(_)) => {}
                Some(TokenTree::Group(_)) => continue 'outer,
                Some(token) => break 'outer Some(token),
            }
        }
    };
    if let Some(token) = first_non_attr_token {
        let msg = match &token {
            TokenTree::Ident(ident) if ident == "impl" => "invalid impl block for #[delegate]",
            TokenTree::Ident(ident) if ident == "fn" => "invalid method for #[delegate]",
            _ => "expected an impl block or method inside impl block",
        };
        quote_spanned! { token.span() => compile_error!(#msg); }
    } else {
        panic!("unexpected eof")
    }
}

fn delegate_impl_block(input: ItemImpl, receiver: &Expr) -> TokenStream {
    let ItemImpl {
        attrs,
        defaultness,
        unsafety,
        impl_token,
        mut generics,
        trait_,
        self_ty,
        brace_token: _,
        items,
    } = input;
    let where_clause = generics.where_clause.take();
    let trait_ = trait_.map(|(bang, path, for_)| quote!(#bang #path #for_));
    let items = items.into_iter().map(|item| {
        let method = match item {
            ImplItem::Method(m) => m,
            _ => return item.into_token_stream(),
        };
        delegate_method(method, receiver)
    });

    quote! {
        #(#attrs)* #defaultness #unsafety #impl_token #generics #trait_ #self_ty #where_clause {
            #(#items)*
        }
    }
}

fn delegate_method(input: ImplItemMethod, receiver: &Expr) -> TokenStream {
    let ImplItemMethod {
        mut attrs,
        vis,
        defaultness,
        sig,
        block: _,
    } = input;
    let mut errors = TokenStream::new();
    let mut push_error = |span: Span, msg: &'static str| {
        errors.extend(quote_spanned! { span => compile_error!(#msg); });
    };
    // Parse attributes.
    let mut has_inline = false;
    let mut has_into = false;
    let mut call_name = None;
    attrs.retain(|attr| {
        let path = &attr.path;
        if path.is_ident("inline") {
            has_inline = true;
        } else if path.is_ident("into") {
            if !attr.tokens.is_empty() {
                push_error(attr.tokens.span(), "unexpected argument");
            }
            if has_into {
                push_error(attr.span(), "duplicate #[into] attribute");
            }
            has_into = true;
            return false;
        } else if path.is_ident("call") {
            match syn::parse2::<ExprParen>(attr.tokens.clone()) {
                Ok(expr) if expr.attrs.is_empty() => {
                    let inner = expr.expr;
                    match &*inner {
                        Expr::Path(path) if path.attrs.is_empty() && path.qself.is_none() => {
                            if let Some(ident) = path.path.get_ident() {
                                if call_name.is_some() {
                                    push_error(attr.span(), "duplicate #[call] attribute");
                                }
                                call_name = Some(ident.clone());
                            } else {
                                push_error(
                                    inner.span(),
                                    "invalid argument, expected an identifier",
                                );
                            }
                        }
                        _ => push_error(inner.span(), "invalid argument, expected an identifier"),
                    }
                }
                _ => push_error(attr.tokens.span(), "invalid argument"),
            }
            return false;
        }
        true
    });
    // Mark method always inline if it's not otherwise specified.
    let inline = if !has_inline {
        quote!(#[inline(always)])
    } else {
        quote!()
    };
    let mut inputs = sig.inputs.iter();
    // Extract the self token.
    let self_token = match inputs.next() {
        Some(FnArg::Receiver(receiver)) => receiver.self_token.to_token_stream(),
        Some(FnArg::Typed(pat)) => match &*pat.pat {
            Pat::Ident(ident) if ident.ident == "self" => ident.ident.to_token_stream(),
            _ => {
                push_error(pat.span(), "expected self");
                TokenStream::new()
            }
        },
        None => {
            push_error(sig.paren_token.span, "expected self");
            TokenStream::new()
        }
    };
    // List all parameters.
    let args = inputs
        .filter_map(|arg| match arg {
            FnArg::Typed(pat) => match &*pat.pat {
                Pat::Ident(ident) => Some(ident.to_token_stream()),
                _ => {
                    push_error(pat.pat.span(), "expect an identifier");
                    None
                }
            },
            _ => {
                push_error(arg.span(), "unexpected argument");
                None
            }
        })
        .collect::<Vec<_>>();
    // Return errors if any.
    if !errors.is_empty() {
        return errors;
    } else {
        // Drop it to ensure that we are not pushing anymore into it.
        drop(errors);
    }
    // Generate method call.
    let name = call_name.as_ref().unwrap_or(&sig.ident);
    // Replace the self token in the receiver with the token we extract above to ensure it comes
    // from the right hygiene context.
    let receiver = replace_self(receiver.to_token_stream(), &self_token);
    let body = quote! { #receiver.#name(#(#args),*) };
    let body = match &sig.output {
        ReturnType::Default => quote! { #body; },
        ReturnType::Type(_, ty) if has_into => {
            quote! { ::std::convert::Into::<#ty>::into(#body) }
        }
        _ => body,
    };
    quote! {
        #(#attrs)* #inline #vis #defaultness #sig {
            #body
        }
    }
}

fn replace_self(expr: TokenStream, self_token: &TokenStream) -> TokenStream {
    expr.into_iter()
        .map(|token| match token {
            TokenTree::Ident(ident) if ident == "self" => self_token.clone(),
            TokenTree::Group(group) => {
                let delimiter = group.delimiter();
                let stream = replace_self(group.stream(), self_token);
                Group::new(delimiter, stream).into_token_stream()
            }
            _ => token.into_token_stream(),
        })
        .collect()
}

pub(crate) fn delegate(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let receiver = parse_macro_input!(attr as Expr);
    delegate_input(item.into(), &receiver).into()
}