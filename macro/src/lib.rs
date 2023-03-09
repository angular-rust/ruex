#![allow(clippy::needless_doctest_main)]
// #![warn(
//     missing_debug_implementations,
//     missing_docs,
//     rust_2018_idioms,
//     unreachable_pub
// )]
#![doc(test(
    no_crate_inject,
    attr(deny(warnings, rust_2018_idioms), allow(dead_code, unused_variables))
))]

use proc_macro::TokenStream;

mod impls;
use impls::{Mode, Type};

const MAX_UDP_PAYLOAD: usize = 65507;

/// Composition root for application entry point.
///
/// Inspired by `#[tokio::main]`
///
/// Implements dependency injection composition root pattern for
/// application entry point.
///
/// ## Examples
///
/// ```
/// #[ruex::main {
///     backtrace = {
///         level = Short,
///     },
///     log = {
///         level = Info,
///         output = Stdout,
///     }
/// }]
/// fn main() {}
/// ```
///
/// ## Config
/// Configuration fields can be omitted.
///
/// ```rust,no_run
/// struct MainConfig {
///     backtrace: BacktraceConfig,
///     log: LogConfig,
/// }
///
/// struct LogConfig {
///     level: LogLevel,
///     output: LogOutput,
/// }
///
/// enum LogOutput {
///     #[default]
///     StdOut,
///     Syslog,
///     File(String),
/// }
///
/// enum LogLevel {
///     Trace,
///     Debug,
///     #[default]
///     Info,
///     Warn,
///     Error,
/// }
///
/// struct BacktraceConfig {
///     level: BactraceLevel,
/// }
///
/// enum BactraceLevel {
///     None,
///     #[default]
///     Short,
///     Full,
/// }
/// ```
#[proc_macro_attribute]
// #[cfg(not(test))] // Work around for rust-lang/rust#62127
pub fn main(attr: TokenStream, item: TokenStream) -> TokenStream {
    impls::main(attr, item)
}

/// Decorate methods.
///
/// Inspired by `deco`
///
/// ## Examples
///
/// ```
/// use ruex::prelude::*;
///
/// fn logging<F>(func: F) -> impl Fn(i32) -> i32
/// where
///     F: Fn(i32) -> i32,
/// {
///     move |i| {
///         println!("Input = {}", i);
///         let out = func(i);
///         println!("Output = {}", out);
///         out
///     }
/// }
///
/// #[decorate(logging)]
/// fn add2(i: i32) -> i32 {
///     i + 2
/// }
///
/// add2(2);
/// ```
///
/// - Decorator with parameter
///
/// ```
/// use ruex::prelude::*;
/// use std::{fs, io::Write};
///
/// fn logging<InputFunc: 'static>(
///     log_filename: &'static str,
/// ) -> impl Fn(InputFunc) -> Box<dyn Fn(i32) -> i32>
/// where
///     InputFunc: Fn(i32) -> i32,
/// {
///     move |func: InputFunc| {
///         Box::new(move |i: i32| {
///             let mut f = fs::File::create(log_filename).unwrap();
///             writeln!(f, "Input = {}", i).unwrap();
///             let out = func(i);
///             writeln!(f, "Output = {}", out).unwrap();
///             out
///         })
///     }
/// }
///
/// #[decorate(logging("test.log"))]
/// fn add2(i: i32) -> i32 {
///     i + 2
/// }
///
/// add2(2);
/// ```
///
#[proc_macro_attribute]
pub fn decorate(attr: TokenStream, item: TokenStream) -> TokenStream {
    impls::decorate(attr, item)
}

/// Delegate method to a field.
///
/// Inspired by delegate-attr
///
/// ## Examples
///
/// ### Delegate `impl` block
///
/// ```
/// use delegate_attr::delegate;
///
/// struct Foo(String);
///
/// #[delegate(self.0)]
/// impl Foo {
///     fn as_str(&self) -> &str;
///     fn into_bytes(self) -> Vec<u8>;
/// }
///
/// let foo = Foo("hello".to_owned());
/// assert_eq!(foo.as_str(), "hello");
/// assert_eq!(foo.into_bytes(), b"hello");
/// ```
///
/// ### Delegate trait `impl`
///
/// ```
/// # use delegate_attr::delegate;
///
/// struct Iter(std::vec::IntoIter<u8>);
///
/// #[delegate(self.0)]
/// impl Iterator for Iter {
///     type Item = u8;
///     fn next(&mut self) -> Option<u8>;
///     fn count(self) -> usize;
///     fn size_hint(&self) -> (usize, Option<usize>);
///     fn last(self) -> Option<u8>;
/// }
///
/// let iter = Iter(vec![1, 2, 4, 8].into_iter());
/// assert_eq!(iter.count(), 4);
/// let iter = Iter(vec![1, 2, 4, 8].into_iter());
/// assert_eq!(iter.last(), Some(8));
/// let iter = Iter(vec![1, 2, 4, 8].into_iter());
/// assert_eq!(iter.sum::<u8>(), 15);
/// ```
///
/// ### With more complicated target
///
/// ```
/// # use delegate_attr::delegate;
/// # use std::cell::RefCell;
/// struct Foo<T> {
///     inner: RefCell<Vec<T>>,
/// }
///
/// #[delegate(self.inner.borrow())]
/// impl<T> Foo<T> {
///     fn len(&self) -> usize;
/// }
///
/// #[delegate(self.inner.borrow_mut())]
/// impl<T> Foo<T> {
///     fn push(&self, value: T);
/// }
///
/// #[delegate(self.inner.into_inner())]
/// impl<T> Foo<T> {
///     fn into_boxed_slice(self) -> Box<[T]>;
/// }
///
/// let foo = Foo { inner: RefCell::new(vec![1]) };
/// assert_eq!(foo.len(), 1);
/// foo.push(2);
/// assert_eq!(foo.len(), 2);
/// assert_eq!(foo.into_boxed_slice().as_ref(), &[1, 2]);
/// ```
///
/// ### `into` and `call` attribute
///
/// ```
/// # use delegate_attr::delegate;
/// struct Inner;
/// impl Inner {
///     pub fn method(&self, num: u32) -> u32 { num }
/// }
///
/// struct Wrapper { inner: Inner }
///
/// #[delegate(self.inner)]
/// impl Wrapper {
///     // calls method, converts result to u64
///     #[into]
///     pub fn method(&self, num: u32) -> u64;
///
///     // calls method, returns ()
///     #[call(method)]
///     pub fn method_noreturn(&self, num: u32);
/// }
/// ```
///
/// ### Delegate single method
///
/// ```
/// # use delegate_attr::delegate;
/// struct Foo<T>(Vec<T>);
///
/// impl<T> Foo<T> {
///     #[delegate(self.0)]
///     fn len(&self) -> usize;
/// }
///
/// let foo = Foo(vec![1]);
/// assert_eq!(foo.len(), 1);
/// ```
#[proc_macro_attribute]
pub fn delegate(attr: TokenStream, item: TokenStream) -> TokenStream {
    impls::delegate(attr, item)
}

/// Elegant trait mocking.
///
/// Use it with #[cfg_attr(test, mock)]
///
/// ## Examples
///
/// ```rust
/// #![allow(unused_variables)]
/// use ruex::prelude::*;
///
/// /// Here
/// #[mock]
/// trait Nurse {
///     fn heal(&self, value: i32, direction: i32) -> i32 {
///         0
///     }
///
///     fn leave(&self, value: i32) -> i32 {
///         0
///     }
/// }
///
/// #[derive(Default)]
/// struct Foo;
///
/// impl Nurse for Foo {
///     fn heal(&self, value: i32, direction: i32) -> i32 {
///         25
///     }
///
///     fn leave(&self, value: i32) -> i32 {
///         31
///     }
/// }
///
/// fn main() {
///     let nurse = Foo::mock().heal(|value, direction| 123).build();
///
///     let val = nurse.heal(23, 0);
///     println!("VALUE: {val}");
///
///     let val = nurse.leave(23);
///     println!("VALUE: {val}");
/// }
/// ```
#[proc_macro_attribute]
pub fn mock(attr: TokenStream, item: TokenStream) -> TokenStream {
    impls::mock(attr, item)
}

/// Subject registration.
#[proc_macro_attribute]
pub fn register(attr: TokenStream, item: TokenStream) -> TokenStream {
    impls::register(attr.into(), item.into()).into()
}

/// Aspect-oriented methodology.
///
#[proc_macro_attribute]
pub fn aspect(attr: TokenStream, item: TokenStream) -> TokenStream {
    impls::contracts_aspect(Type::Aspect, Mode::Always, attr.into(), item.into()).into()
}

/// Composition pattern implementation.
///
#[proc_macro_derive(Compose, attributes(delegate))]
pub fn derive_compose(item: TokenStream) -> TokenStream {
    impls::compose(item.into()).into()
}

/// Debug ensures for contracts.
///
#[proc_macro_attribute]
pub fn debug_ensures(attr: TokenStream, item: TokenStream) -> TokenStream {
    impls::contracts_aspect(Type::Ensures, Mode::Debug, attr.into(), item.into()).into()
}

/// Debug invariant for contracts.
///
#[proc_macro_attribute]
pub fn debug_invariant(attr: TokenStream, item: TokenStream) -> TokenStream {
    impls::contracts_aspect(Type::Invariant, Mode::Debug, attr.into(), item.into()).into()
}

/// Debug requires for contracts.
///
#[proc_macro_attribute]
pub fn debug_requires(attr: TokenStream, item: TokenStream) -> TokenStream {
    impls::contracts_aspect(Type::Requires, Mode::Debug, attr.into(), item.into()).into()
}

/// Ensures for contracts.
///
#[proc_macro_attribute]
pub fn ensures(attr: TokenStream, item: TokenStream) -> TokenStream {
    impls::contracts_aspect(Type::Ensures, Mode::Always, attr.into(), item.into()).into()
}

/// Invariant for contracts.
///
#[proc_macro_attribute]
pub fn invariant(attr: TokenStream, item: TokenStream) -> TokenStream {
    impls::contracts_aspect(Type::Invariant, Mode::Always, attr.into(), item.into()).into()
}

/// Requires for contracts.
///
#[proc_macro_attribute]
pub fn requires(attr: TokenStream, item: TokenStream) -> TokenStream {
    impls::contracts_aspect(Type::Requires, Mode::Always, attr.into(), item.into()).into()
}

/// Test ensures for contracts.
///
#[proc_macro_attribute]
pub fn test_ensures(attr: TokenStream, item: TokenStream) -> TokenStream {
    impls::contracts_aspect(Type::Ensures, Mode::Test, attr.into(), item.into()).into()
}

/// Test invariant for contracts.
///
#[proc_macro_attribute]
pub fn test_invariant(attr: TokenStream, item: TokenStream) -> TokenStream {
    impls::contracts_aspect(Type::Invariant, Mode::Test, attr.into(), item.into()).into()
}

/// Test requires for contracts.
///
#[proc_macro_attribute]
pub fn test_requires(attr: TokenStream, item: TokenStream) -> TokenStream {
    impls::contracts_aspect(Type::Requires, Mode::Test, attr.into(), item.into()).into()
}
