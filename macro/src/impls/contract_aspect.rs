use std::{
    collections::{HashMap, VecDeque},
    fmt,
    net::UdpSocket,
};

// use proc_macro::TokenStream;
use proc_macro2::{Punct, Spacing, TokenStream as TokenStream2, TokenTree};
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream, Result},
    punctuated::Punctuated,
    visit_mut::VisitMut,
    Attribute, Expr, ExprLit, ExprPath, Lit, Meta, MetaNameValue, PathSegment, Token, TraitItem,
    TraitItemMethod,
};

use companion::{companion_addr, Response, Task};

use crate::MAX_UDP_PAYLOAD;

pub struct AspectJointPoint<'a> {
    pub stream: &'a TokenStream2,
}

impl<'a> VisitMut for AspectJointPoint<'a> {
    fn visit_expr_mut(&mut self, item: &mut Expr) {
        if let Expr::Call(ref node) = item {
            if let Expr::Path(call) = node.func.as_ref() {
                let segments = &call.path.segments;
                if segments.len() == 2 {
                    if let (Some(first), Some(last)) = (segments.first(), segments.last()) {
                        if first.ident.to_string() == "AspectJointPoint"
                            && last.ident.to_string() == "proceed"
                        {
                            // prepare variables
                            let stream = self.stream;
                            *item = syn::parse_quote!(#stream);
                        }
                    }
                }
            }
        }

        // Delegate to the default impl to visit nested expressions.
        syn::visit_mut::visit_expr_mut(self, item);
    }
}

#[derive(Default, Debug)]
pub struct Aspect {
    pub name: String,
    pub docs: VecDeque<String>,
    pub before: Option<TraitItemMethod>,
    pub after: Option<TraitItemMethod>,
    pub around: Option<TraitItemMethod>,
}

impl Aspect {
    pub fn new(attr: TokenStream2) -> Self {
        // prefetch from attrs
        match syn::parse2::<syn::TypePath>(attr.clone()) {
            Ok(_) => {}
            Err(_) => panic!("Usage: #[aspect(std::fmt::Display)]"),
        }

        let key = attr.to_string().replace(" :: ", "::");

        let addr = companion_addr();

        let socket = UdpSocket::bind("[::]:0").unwrap();
        socket.connect(addr).unwrap();
        let mut buf = [0; MAX_UDP_PAYLOAD];

        socket.send(&Task::Get(&key).as_bytes()).unwrap();
        let (len, _src) = socket.recv_from(&mut buf).unwrap();
        let resp = Response::from(&buf[..len]);

        if let Response::String(data) = resp {
            let item_trait: syn::ItemTrait = syn_serde::json::from_str(&data).unwrap();
            let mut aspect = Aspect::default();
            aspect.name = key;
            aspect.docs = collect_docs(&item_trait.attrs);

            // construct proto
            for item in item_trait.items.iter() {
                match item {
                    TraitItem::Method(method) => {
                        // here
                        let name = method.sig.ident.to_string();
                        // println!("Sig {}", name);
                        match name.as_str() {
                            "before" => aspect.before = Some(method.clone()),
                            "after" => aspect.after = Some(method.clone()),
                            "around" => aspect.around = Some(method.clone()),
                            _ => {
                                println!("Incompatible Aspect trait")
                            }
                        }
                    }
                    _ => {
                        println!("Incompatible Aspect trait")
                    }
                }
            }

            aspect
        } else {
            panic!("Aspect is not registered")
        }
    }

    pub fn documentation(&self) -> VecDeque<String> {
        let mut output: VecDeque<String> = VecDeque::new();

        let mut before_docs: VecDeque<String> = VecDeque::new();
        let mut after_docs: VecDeque<String> = VecDeque::new();
        let mut around_docs: VecDeque<String> = VecDeque::new();

        if let Some(ref method) = self.before {
            before_docs = collect_docs(&method.attrs);
        }

        if let Some(ref method) = self.around {
            around_docs = collect_docs(&method.attrs);
        }

        if let Some(ref method) = self.after {
            after_docs = collect_docs(&method.attrs);
        }

        // if there something in documentations
        if !self.docs.is_empty()
            || !before_docs.is_empty()
            || !after_docs.is_empty()
            || !around_docs.is_empty()
        {
            output.push_back(format!(" ### {}", self.name));
            self.docs.iter().for_each(|item| {
                output.push_back(item.clone());
            });

            if !before_docs.is_empty() {
                // output.push_back(String::from(""));
                output.push_back(String::from(" - __Before:__"));

                before_docs.iter().for_each(|item| {
                    output.push_back(item.clone());
                });
            }

            if !around_docs.is_empty() {
                // output.push_back(String::from(""));
                output.push_back(String::from(" - __Around:__"));

                around_docs.iter().for_each(|item| {
                    output.push_back(item.clone());
                });
            }

            if !after_docs.is_empty() {
                // output.push_back(String::from(""));
                output.push_back(String::from(" - __After:__"));

                after_docs.iter().for_each(|item| {
                    output.push_back(item.clone());
                });
            }
        }

        output
    }
}

fn collect_docs(attrs: &Vec<Attribute>) -> VecDeque<String> {
    let mut docs: VecDeque<String> = VecDeque::new();
    attrs.iter().for_each(|attr| {
        // Collect docs attributes
        if attr.path.is_ident("doc") {
            match attr.parse_meta().unwrap() {
                Meta::NameValue(MetaNameValue {
                    lit: Lit::Str(lit_str),
                    ..
                }) => {
                    docs.push_back(lit_str.value());
                }
                _ => {}
            }
        }
    });
    docs
}

// detect ranges separated by implication and return
fn stream_ranges(input: &Vec<TokenTree>) -> Vec<(usize, usize)> {
    let mut ranges: Vec<(usize, usize)> = vec![];

    let mut tail = &TokenTree::Punct(Punct::new(' ', Spacing::Alone));
    let mut start_current = 0;

    // split for  ->
    for (idx, attr) in input.iter().enumerate() {
        if let TokenTree::Punct(cur) = attr {
            if cur.as_char() == '>' && cur.spacing() == Spacing::Alone {
                if let TokenTree::Punct(prev) = tail {
                    if prev.as_char() == '-' && prev.spacing() == Spacing::Joint {
                        ranges.push((start_current, idx - 1));
                        start_current = idx + 1;
                    }
                }
            }
        }

        tail = attr;
    }

    if start_current < input.len() {
        ranges.push((start_current, input.len()));
    }

    ranges
}

/// Checking-mode of a contract.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Mode {
    /// Always check contract
    Always,
    /// Never check contract
    Disabled,
    /// Check contract only in debug builds
    Debug,
    /// Check contract only in `#[cfg(test)]` configurations
    Test,
    /// Check the contract and print information upon violation, but don't abort
    /// the program.
    LogOnly,
}

impl Mode {
    /// Return the prefix of attributes of `self` mode.
    pub fn name(self) -> Option<&'static str> {
        match self {
            Mode::Always => Some(""),
            Mode::Disabled => None,
            Mode::Debug => Some("debug_"),
            Mode::Test => Some("test_"),
            Mode::LogOnly => None,
        }
    }

    /// Computes the contract type based on feature flags.
    pub fn final_mode(self) -> Self {
        // disabled ones can't be "forced", test ones should stay test, no
        // matter what.
        if self == Mode::Disabled || self == Mode::Test {
            return self;
        }

        if cfg!(feature = "disable_contracts") {
            Mode::Disabled
        } else if cfg!(feature = "override_debug") {
            // log is "weaker" than debug, so keep log
            if self == Mode::LogOnly {
                self
            } else {
                Mode::Debug
            }
        } else if cfg!(feature = "override_log") {
            Mode::LogOnly
        } else {
            self
        }
    }
}

/// The different contract types.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Type {
    Requires,
    Ensures,
    Invariant,
    Aspect,
}

impl Type {
    /// Get the name that is used as a message-prefix on violation of a
    /// contract.
    pub fn message_name(self) -> &'static str {
        match self {
            Type::Requires => "Pre-condition",
            Type::Ensures => "Post-condition",
            Type::Invariant => "Invariant",
            Type::Aspect => "Invariant",
        }
    }

    /// Determine the type and mode of an identifier.
    pub fn type_and_mode(ident: &str) -> Option<(Type, Mode)> {
        match ident {
            "aspect" => Some((Type::Aspect, Mode::Always)),
            "requires" => Some((Type::Requires, Mode::Always)),
            "ensures" => Some((Type::Ensures, Mode::Always)),
            "invariant" => Some((Type::Invariant, Mode::Always)),
            "debug_requires" => Some((Type::Requires, Mode::Debug)),
            "debug_ensures" => Some((Type::Ensures, Mode::Debug)),
            "debug_invariant" => Some((Type::Invariant, Mode::Debug)),
            "test_requires" => Some((Type::Requires, Mode::Test)),
            "test_ensures" => Some((Type::Ensures, Mode::Test)),
            "test_invariant" => Some((Type::Invariant, Mode::Test)),
            _ => None,
        }
    }
}

// Contain single part of contract case as vector of Rule expressions
#[derive(Debug)]
pub struct CaseRule(Vec<RuleExpression>);

impl CaseRule {
    fn add(&mut self, value: RuleExpression) {
        self.0.push(value);
    }
}

impl Default for CaseRule {
    fn default() -> Self {
        Self(Default::default())
    }
}

// Describe rule expression
#[derive(Debug)]
enum RuleExpression {
    If(Expr),
    Expr(Expr),
    // Desc(String),
}

pub struct Contract {
    pub ty: Type,
    pub mode: Mode,
    pub desc: Option<String>,
    pub rules: Vec<CaseRule>,
    // variable declarations
    pub decls: HashMap<String, Expr>,
}

impl Contract {
    pub fn syntax(&self) -> Vec<TokenStream2> {
        let assert = match self.mode {
            Mode::Always => format_ident!("assert"),
            Mode::Debug | Mode::Test => {
                format_ident!("debug_assert")
            }
            Mode::Disabled | Mode::LogOnly => return vec![],
        };

        self.rules
            .iter()
            .map(|rule| {
                let mut holder = quote! {};

                for expr in rule.0.iter().rev() {
                    match expr {
                        RuleExpression::If(e) => {
                            holder = quote! {
                                if (#e) {
                                    #holder
                                }
                            };
                        }
                        RuleExpression::Expr(e) => match self.desc {
                            Some(ref desc) => {
                                holder = quote! {
                                    #assert!(#e, #desc);
                                };
                            }
                            None => {
                                holder = quote! {
                                    #assert!(#e);
                                };
                            }
                        },
                    }
                }
                holder
            })
            .collect()
    }
}

impl fmt::Debug for Contract {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.desc {
            Some(desc) => {
                write!(
                    f,
                    "{:?}:{:?}, {} rules [{desc}]",
                    self.mode,
                    self.ty,
                    self.rules.len()
                )
            }
            None => {
                write!(
                    f,
                    "{:?}:{:?}, {} rules",
                    self.mode,
                    self.ty,
                    self.rules.len()
                )
            }
        }
    }
}

impl Contract {
    pub fn new(ty: Type, mode: Mode) -> Self {
        Self {
            ty,
            mode,
            desc: None,
            rules: Vec::new(),
            decls: HashMap::new(),
        }
    }

    // Parse attribures from TokenStream into rules
    //
    pub fn parse_attributes(&mut self, stream: TokenStream2) {
        // should split stream by implication first, instead coma
        let stream = stream.into_iter().collect::<Vec<_>>();

        let ranges = stream_ranges(&stream);

        // `peekable` to look at the next element of the iterator without consuming it
        let mut range_iter = ranges.iter().peekable();

        // expressions placeholder
        let mut rule = CaseRule::default();
        while let Some(range) = range_iter.next() {
            let stream = stream[range.0..range.1]
                .iter()
                .map(|x| x.clone())
                .collect::<TokenStream2>();
            let Segment { expressions } = syn::parse2(stream).unwrap();

            let is_last_segment = range_iter.peek().is_none();

            let mut expr_iter = expressions.iter().peekable();
            while let Some(expr) = expr_iter.next() {
                let is_last_expr = expr_iter.peek().is_none();

                if !is_last_segment && is_last_expr {
                    match expr {
                        Expr::Lit(_) => panic!("Description should be last element"),
                        _ => {
                            let mut expr = expr.clone();
                            let mut replacer = OldPseudo::default();
                            replacer.visit_expr_mut(&mut expr);

                            if self.ty != Type::Ensures && !replacer.items.is_empty() {
                                panic!("Only ensures support 'old' pseudo-expressions")
                            }

                            replacer.items.into_iter().for_each(|(k, v)| {
                                self.decls.insert(k, v);
                            });

                            rule.add(RuleExpression::If(expr));
                        }
                    }
                } else if is_last_segment && !is_last_expr {
                    match expr {
                        Expr::Lit(_) => panic!("Description should be last element"),
                        _ => {
                            let mut expr = expr.clone();
                            let mut replacer = OldPseudo::default();
                            replacer.visit_expr_mut(&mut expr);

                            if self.ty != Type::Ensures && !replacer.items.is_empty() {
                                panic!("Only ensures support 'old' pseudo-expressions")
                            }

                            replacer.items.into_iter().for_each(|(k, v)| {
                                self.decls.insert(k, v);
                            });

                            rule.add(RuleExpression::Expr(expr));
                            self.rules.push(rule);
                            rule = CaseRule::default();
                        }
                    }
                } else if is_last_segment && is_last_expr {
                    // get desc if exists
                    match expr {
                        Expr::Lit(ExprLit {
                            lit: Lit::Str(token),
                            ..
                        }) => {
                            self.desc = Some(token.value());
                        }
                        _ => {
                            let mut expr = expr.clone();
                            let mut replacer = OldPseudo::default();
                            replacer.visit_expr_mut(&mut expr);

                            if self.ty != Type::Ensures && !replacer.items.is_empty() {
                                panic!("Only ensures support 'old' pseudo-expressions")
                            }

                            replacer.items.into_iter().for_each(|(k, v)| {
                                self.decls.insert(k, v);
                            });

                            rule.add(RuleExpression::Expr(expr));
                            self.rules.push(rule);
                            rule = CaseRule::default();
                        }
                    }
                } else {
                    match expr {
                        Expr::Lit(_) => panic!("Description should be last element"),
                        _ => {
                            let mut expr = expr.clone();
                            let mut replacer = OldPseudo::default();
                            replacer.visit_expr_mut(&mut expr);

                            if self.ty != Type::Ensures && !replacer.items.is_empty() {
                                panic!("Only ensures support 'old' pseudo-expressions")
                            }

                            replacer.items.into_iter().for_each(|(k, v)| {
                                self.decls.insert(k, v);
                            });

                            rule.add(RuleExpression::Expr(expr));
                            self.rules.push(rule);
                            rule = CaseRule::default();
                        }
                    }
                }
            }
        }
    }
}

// faster trim whitespace
pub fn trim_whitespace(s: &str) -> String {
    let mut new_str = s.trim().to_owned();
    let mut prev = ' '; // The initial value doesn't really matter
    new_str.retain(|ch| {
        let result = ch != ' ' || prev != ' ';
        prev = ch;
        result
    });
    new_str
}

#[derive(Debug, Default)]
struct OldPseudo {
    items: HashMap<String, Expr>,
}

impl VisitMut for OldPseudo {
    fn visit_expr_mut(&mut self, item: &mut Expr) {
        if let Expr::Call(ref node) = item {
            if let syn::Expr::Path(call) = node.func.as_ref() {
                let segments = &call.path.segments;
                if let Some(path) = segments.first() {
                    if path.ident.to_string() == "old" {
                        // println!("Call: {item:?}");
                        let name = format!("{}", quote!(#node));
                        let cleaned: String = name
                            .chars()
                            .filter_map(|x| match x {
                                'A'..='Z' | 'a'..='z' | '0'..='9' | ' ' => Some(x),
                                _ => None,
                            })
                            .collect();

                        let name = trim_whitespace(&cleaned)
                            .split(' ')
                            .filter(|s| !s.is_empty())
                            .collect::<Vec<_>>()
                            .join("_");

                        let var = format_ident!("{name}");

                        let old_arg = node
                            .args
                            .first()
                            .expect("The 'old' pseudo-function have exactly one parameter");

                        // store node as `old` pseudo-expressions
                        self.items.insert(name, old_arg.clone());

                        let expr: ExprPath = syn::parse_quote!(#var);
                        *item = Expr::Path(expr);
                    }
                }
            }
        }
        // Delegate to the default impl to visit nested expressions.
        syn::visit_mut::visit_expr_mut(self, item);
    }
}

#[derive(Debug, Clone)]
pub struct Segment {
    pub expressions: Vec<Expr>,
}

impl Parse for Segment {
    fn parse(input: ParseStream) -> Result<Self> {
        let vars = Punctuated::<Expr, Token![,]>::parse_terminated(input).unwrap();
        let expressions: Vec<Expr> = vars.into_iter().collect();

        // println!("{items:#?}");
        Ok(Segment { expressions })
    }
}

#[derive(Default, Debug)]
pub struct ContractAspectState {
    pub docs: VecDeque<String>,
    pub requires: Vec<Contract>,
    pub invariants: Vec<Contract>,
    pub ensures: Vec<Contract>,
    pub aspects: Vec<Aspect>,
}

impl ContractAspectState {
    // Process rest of contract attributes
    // and keep others, to generate new attributes set
    pub fn process(&mut self, attrs: &Vec<Attribute>) -> Vec<Attribute> {
        attrs
            .iter()
            .filter_map(|attr| {
                // let len = attr.path.segments.len();
                if let Some(PathSegment { ident, .. }) = attr.path.segments.last() {
                    let pair = Type::type_and_mode(&ident.to_string());
                    if let Some((ty, mode)) = pair {
                        // create case
                        match ty {
                            Type::Requires | Type::Ensures | Type::Invariant => {
                                let mut case = Contract::new(ty, mode);
                                let tokens = attr.tokens.clone().into_iter().collect::<Vec<_>>();
                                assert!(
                                    tokens.len() == 1,
                                    "Wrong contract sintax: not parenthesized expression"
                                );

                                if let Some(TokenTree::Group(group)) = tokens.first() {
                                    case.parse_attributes(group.stream());
                                    match ty {
                                        Type::Requires => self.requires.push(case),
                                        Type::Ensures => self.ensures.push(case),
                                        Type::Invariant => self.invariants.push(case),
                                        _ => unreachable!(),
                                    }
                                } else {
                                    panic!("Wrong contract sintax: not parenthesized expression")
                                }
                            }
                            Type::Aspect => {
                                let tokens = attr.tokens.clone().into_iter().collect::<Vec<_>>();
                                assert!(
                                    tokens.len() == 1,
                                    "Wrong aspect sintax: not parenthesized expression"
                                );
                                if let Some(TokenTree::Group(group)) = tokens.first() {
                                    let aspect = Aspect::new(group.stream());
                                    self.aspects.push(aspect);
                                } else {
                                    panic!("Wrong contract sintax: not parenthesized expression")
                                }
                            }
                        };

                        // Eat contract attribute
                        None
                    } else {
                        // Attribute is not contract
                        // Collect docs attributes
                        if attr.path.is_ident("doc") {
                            match attr.parse_meta().unwrap() {
                                Meta::NameValue(MetaNameValue {
                                    lit: Lit::Str(lit_str),
                                    ..
                                }) => {
                                    // println!("{}", lit_str.value());
                                    self.docs.push_back(lit_str.value());
                                    None
                                }
                                _ => Some(attr.clone()),
                            }
                        } else {
                            Some(attr.clone())
                        }
                    }
                } else {
                    // Attribute without path
                    unreachable!()
                }
            })
            .collect::<Vec<_>>()
    }

    // collect variable declarations from ensure cases
    pub fn variables(&self) -> Vec<TokenStream2> {
        let mut decls: HashMap<String, Expr> = HashMap::new();
        self.ensures.iter().for_each(|case| {
            case.decls.iter().for_each(|(name, expr)| {
                decls.insert(name.to_string(), expr.clone());
            });
        });

        decls
            .into_iter()
            .map(|(name, expr)| {
                let name = format_ident!("{name}");
                quote! {
                    let #name = #expr;
                }
            })
            .collect::<Vec<_>>()
    }
}

pub trait SyntaxAndDocs {
    fn generate(&self) -> (Vec<TokenStream2>, Vec<String>);
}

impl SyntaxAndDocs for Vec<Contract> {
    fn generate(&self) -> (Vec<TokenStream2>, Vec<String>) {
        let mut docs = vec![];
        let stream = self
            .iter()
            .map(|case| {
                if let Some(ref desc) = case.desc {
                    docs.push(format!(" - {}", desc))
                }
                case.syntax()
            })
            .flatten()
            .collect::<Vec<_>>();
        (stream, docs)
    }
}

// process macro attribute by initiating currentt case and handle rest of cases
pub(crate) fn contracts_aspect(ty: Type, mode: Mode, attrs: TokenStream2, item: TokenStream2) -> TokenStream2 {
    let mut state: ContractAspectState = ContractAspectState::default();

    // process primary contract case

    match ty {
        Type::Requires | Type::Ensures | Type::Invariant => {
            let mut case = Contract::new(ty, mode);
            case.parse_attributes(attrs);

            match ty {
                Type::Requires => state.requires.push(case),
                Type::Ensures => state.ensures.push(case),
                Type::Invariant => state.invariants.push(case),
                _ => unreachable!(),
            }
        }
        Type::Aspect => {
            state.aspects.push(Aspect::new(attrs));
        }
    }

    // process rest of contract cases
    let input: syn::ItemFn = syn::parse2(item.clone().into()).unwrap();

    let syn::ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = input;

    let attrs = state.process(&attrs);

    let variables = state.variables();

    // генерация синтаксиса и документации
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
            if let Some(item) = &aspect.before && let Some(block) = &item.default {
            let stmts = &block.stmts;
            Some(quote!{
                #(#stmts)*
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
            if let Some(item) = &aspect.after && let Some(block) = &item.default {
                let stmts = &block.stmts;
                Some(quote!{
                    #(#stmts)*
                })
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    // Here we should deal with call name )))
    // For traits we need only change some in output
    let mut around = quote! {inner()};
    let mut has_around = false;

    let mut it = state.aspects.iter().rev().peekable();
    while let Some(aspect) = it.next() {
        if let Some(item) = &aspect.around && let Some(block) = &item.default {

            let mut replacer = AspectJointPoint {
                stream: &around,
            };
            
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
        // TODO: Aspects
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

    let stmts = &block.stmts;
    let result = if has_around {
        quote! {
            let result = {
                #around
            };
        }
    } else {
        quote! {
            let result = inner();
        }
    };

    quote! {
        #(#attrs)* #vis #sig {

            #(#requires)*
            #(#invariants)*

            #(#variables)*

            let inner = || {
                #(#stmts)*
            };

            #(#before)*

            #result

            #(#after)*

            #(#invariants)*
            #(#ensures)*

            result
        }
    }
}