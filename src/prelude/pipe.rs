/// Helper trait to call free functions using method call syntax.
///
/// Rust already allows calling methods using function call syntax, but not the other way around.
/// This crate fills the gap by providing a simple helper trait `Pipe`.
/// 
/// Inspired by https://github.com/xzfc/ufcs.rs, https://github.com/ear7h/plumb
/// and borrowed from https://github.com/KSXGitHub/pipe-trait.git
///
/// ## See also
///
/// Roughtly the same feature is either implemented or proposed in various languages.
///
/// ### Rust
///
/// * [`pipeline.rs`](https://github.com/johannhof/pipeline.rs): implemented as an macro
/// * [RFC #289](https://github.com/rust-lang/rfcs/issues/289): Unified function / method call syntax
/// * [RFC #2049](https://github.com/rust-lang/rfcs/issues/2049): Piping data to functions
/// * [Method-cascading and pipe-forward operators proposal](https://internals.rust-lang.org/t/method-cascading-and-pipe-forward-operators-proposal/7384/59)
/// * [Rust #44874](https://github.com/rust-lang/rust/issues/44874): Tracking issue for `arbitrary_self_types`
///
/// ### Other languages
///
/// * https://en.wikipedia.org/wiki/Uniform_Function_Call_Syntax
/// * [Nim], [DLang]: built-in UFCS
/// * F#, [Elixir]: the pipe operator `|>`
/// * C++: [proposed](https://brevzin.github.io/c++/2019/04/13/ufcs-history/)
/// * [Nim]: https://nim-lang.org/docs/manual.html#procedures-method-call-syntax
/// * [DLang]: https://tour.dlang.org/tour/en/gems/uniform-function-call-syntax-ufcs
/// * [Elixir]: https://elixirschool.com/en/lessons/basics/pipe-operator/
/// 
/// Make it possible to chain regular functions.
///
/// **API Overview:**
///
/// By adding `use ruex::prelude::*`, 9 methods are added to all types:
///
/// |        identifier       |       pipe syntax      |  traditional syntax |
/// |:-----------------------:|:----------------------:|:-------------------:|
/// | `Pipe::pipe`            | `x.pipe(f)`            | `f(x)`              |
/// | `Pipe::pipe_ref`        | `x.pipe_ref(f)`        | `f(&x)`             |
/// | `Pipe::pipe_mut`        | `x.pipe_mut(f)`        | `f(&mut x)`         |
/// | `Pipe::pipe_as_ref`     | `x.pipe_as_ref(f)`     | `f(x.as_ref())`     |
/// | `Pipe::pipe_as_mut`     | `x.pipe_as_mut(f)`     | `f(x.as_mut())`     |
/// | `Pipe::pipe_deref`      | `x.pipe_deref(f)`      | `f(&x)`             |
/// | `Pipe::pipe_deref_mut`  | `x.pipe_deref_mut(f)`  | `f(&mut x)`         |
/// | `Pipe::pipe_borrow`     | `x.pipe_borrow(f)`     | `f(x.borrow())`     |
/// | `Pipe::pipe_borrow_mut` | `x.pipe_borrow_mut(f)` | `f(x.borrow_mut())` |
///
/// **Example:** Same type
///
/// ```rust
/// use ruex::prelude::*;
/// let inc = |x| x + 1;
/// let double = |x| x + x;
/// let square = |x| x * x;
/// let a = (123i32).pipe(inc).pipe(double).pipe(square);
/// let b = square(double(inc(123i32)));
/// assert_eq!(a, b);
/// ```
///
/// **Example:** Type transformation
///
/// ```rust
/// use ruex::prelude::*;
/// let x = 'x';
/// let a = x
///     .pipe(|x| (x, x, x)) // (char, char, char)
///     .pipe(|x| [x, x]) // [(char, char, char); 2]
///     .pipe(|x| format!("{:?}", x)); // String
/// let b = "[('x', 'x', 'x'), ('x', 'x', 'x')]";
/// assert_eq!(a, b);
/// ```
///
/// **Example:** Pipe amongst method chain
///
/// ```rust
/// # async {
/// # use std::fmt::*;
/// # use futures::future::*;
/// # #[derive(Debug, Copy, Clone)]
/// # struct Num(pub i32);
/// # impl Num {
/// #     pub fn inc(&self) -> Self { Self(self.0 + 1) }
/// #     pub fn double(&self) -> Self { Self(self.0 * 2) }
/// #     pub fn square(&self) -> Self { Self(self.0 * self.0) }
/// #     pub fn get(&self) -> i32 { self.0 }
/// #     pub fn future(self) -> Ready<Self> { ready(self) }
/// # }
/// # let my_future = Num(12).future();
/// use ruex::prelude::*;
/// fn log<X: Debug>(x: X) -> X {
///     println!("value: {:?}", x);
///     x
/// }
/// my_future
///     .pipe(log)
///     .await
///     .pipe(log)
///     .inc()
///     .pipe(log)
///     .double()
///     .pipe(log)
///     .square()
///     .pipe(log)
///     .get()
///     .pipe(log);
/// # };
/// ```
///
/// **Example:** Explicit type annotation
///
/// ```rust
/// use ruex::prelude::*;
/// let x = "abc".to_string();
/// let a = x
///     .pipe_ref::<&str, _>(AsRef::as_ref)
///     .chars()
///     .pipe::<Box<_>, _>(Box::new)
///     .collect::<Vec<_>>();
/// let b = vec!['a', 'b', 'c'];
/// assert_eq!(a, b);
/// ```
// #![no_std]
use std::{
    borrow::{Borrow, BorrowMut},
    ops::{Deref, DerefMut},
};

/// All sized types implement this trait.
pub trait Pipe {
    /// Apply `f` to `self`.
    ///
    /// ```
    /// # #[derive(Debug, PartialEq, Eq)]
    /// # struct Foo(i32);
    /// # fn double(x: i32) -> i32 { x * 2 }
    /// # use ruex::prelude::*;
    /// assert_eq!(
    ///     12.pipe(double).pipe(Foo),
    ///     Foo(double(12)),
    /// )
    /// ```
    #[inline]
    fn pipe<R, F>(self, f: F) -> R
    where
        Self: Sized,
        F: FnOnce(Self) -> R,
    {
        f(self)
    }

    /// Apply `f` to `&self`.
    ///
    /// ```
    /// # use ruex::prelude::*;
    /// #[derive(Debug, PartialEq, Eq)]
    /// struct Foo(i32);
    /// let a = Foo(12);
    /// let b = a
    ///     .pipe_ref(|a| a.0) // a is not moved
    ///     .pipe(Foo);
    /// assert_eq!(a, b); // a is used again
    /// ```
    #[inline]
    fn pipe_ref<'a, R, F>(&'a self, f: F) -> R
    where
        F: FnOnce(&'a Self) -> R,
    {
        f(self)
    }

    /// Apply `f` to `&mut self`.
    ///
    /// ```
    /// # use ruex::prelude::*;
    /// #[derive(Debug, PartialEq, Eq)]
    /// struct Foo(i32, i32);
    /// let mut a = Foo(0, 0);
    /// a.pipe_mut(|a| a.0 = 12);
    /// a.pipe_mut(|a| a.1 = 34);
    /// assert_eq!(a, Foo(12, 34));
    /// ```
    #[inline]
    fn pipe_mut<'a, R, F>(&'a mut self, f: F) -> R
    where
        F: FnOnce(&'a mut Self) -> R,
    {
        f(self)
    }

    /// Apply `f` to `&self` where `f` takes a single parameter of type `Param`
    /// and `Self` implements trait [`AsRef<Param>`].
    ///
    /// ```
    /// # use ruex::prelude::*;
    /// fn uppercase(x: &str) -> String {
    ///   x.to_uppercase()
    /// }
    /// let x: String = "abc".to_string();
    /// let y: String = x.pipe_as_ref(uppercase);
    /// assert_eq!(y, "ABC");
    /// ```
    #[inline]
    fn pipe_as_ref<'a, P, R, F>(&'a self, f: F) -> R
    where
        Self: AsRef<P>,
        P: ?Sized + 'a,
        F: FnOnce(&'a P) -> R,
    {
        f(self.as_ref())
    }

    /// Apply `f` to `&mut self` where `f` takes a single parameter of type `Param`
    /// and `Self` implements trait [`AsMut<Param>`].
    ///
    /// ```
    /// # use ruex::prelude::*;
    /// fn modify(target: &mut [i32]) {
    ///   target[0] = 123;
    /// }
    /// let mut vec: Vec<i32> = vec![0, 1, 2, 3];
    /// vec.pipe_as_mut(modify);
    /// assert_eq!(vec, vec![123, 1, 2, 3]);
    /// ```
    #[inline]
    fn pipe_as_mut<'a, P, R, F>(&'a mut self, f: F) -> R
    where
        Self: AsMut<P>,
        P: ?Sized + 'a,
        F: FnOnce(&'a mut P) -> R,
    {
        f(self.as_mut())
    }

    /// Apply `f` to `&self` where `f` takes a single parameter of type `Param`
    /// and `Self` implements trait `Deref<Target = Param>`.
    ///
    /// ```
    /// # use ruex::prelude::*;
    /// fn uppercase(x: &str) -> String {
    ///   x.to_uppercase()
    /// }
    /// let x: String = "abc".to_string();
    /// let y: String = x.pipe_deref(uppercase);
    /// assert_eq!(y, "ABC");
    /// ```
    #[inline]
    fn pipe_deref<'a, Param, R, F>(&'a self, f: F) -> R
    where
        Self: Deref<Target = Param>,
        Param: ?Sized + 'a,
        F: FnOnce(&'a Param) -> R,
    {
        f(self)
    }

    /// Apply `f` to `&mut self` where `f` takes a single parameter of type `Param`
    /// and `Self` implements trait [`DerefMut<Target = Param>`].
    ///
    /// ```
    /// # use ruex::prelude::*;
    /// fn modify(target: &mut [i32]) {
    ///   target[0] = 123;
    /// }
    /// let mut vec: Vec<i32> = vec![0, 1, 2, 3];
    /// vec.pipe_deref_mut(modify);
    /// assert_eq!(vec, vec![123, 1, 2, 3]);
    /// ```
    #[inline]
    fn pipe_deref_mut<'a, Param, R, F>(&'a mut self, f: F) -> R
    where
        Self: DerefMut<Target = Param>,
        Param: ?Sized + 'a,
        F: FnOnce(&'a mut Param) -> R,
    {
        f(self)
    }

    /// Apply `f` to `&self` where `f` takes a single parameter of type `Param`
    /// and `Self` implements trait [`Borrow<Param>`].
    ///
    /// ```
    /// # use ruex::prelude::*;
    /// fn uppercase(x: &str) -> String {
    ///   x.to_uppercase()
    /// }
    /// let x: String = "abc".to_string();
    /// let y: String = x.pipe_borrow(uppercase);
    /// assert_eq!(y, "ABC");
    /// ```
    #[inline]
    fn pipe_borrow<'a, Param, R, F>(&'a self, f: F) -> R
    where
        Self: Borrow<Param>,
        Param: ?Sized + 'a,
        F: FnOnce(&'a Param) -> R,
    {
        f(self.borrow())
    }

    /// Apply `f` to `&mut self` where `f` takes a single parameter of type `Param`
    /// and `Self` implements trait [`BorrowMut<Param>`].
    ///
    /// ```
    /// # use ruex::prelude::*;
    /// fn modify(target: &mut [i32]) {
    ///   target[0] = 123;
    /// }
    /// let mut vec: Vec<i32> = vec![0, 1, 2, 3];
    /// vec.pipe_borrow_mut(modify);
    /// assert_eq!(vec, vec![123, 1, 2, 3]);
    /// ```
    #[inline]
    fn pipe_borrow_mut<'a, Param, R, F>(&'a mut self, f: F) -> R
    where
        Self: BorrowMut<Param>,
        Param: ?Sized + 'a,
        F: FnOnce(&'a mut Param) -> R,
    {
        f(self.borrow_mut())
    }
}
impl<T> Pipe for T {}

#[cfg(test)]
mod tests {
    use futures::executor::block_on;
    use futures::future::lazy;

    use super::Pipe;

    async fn async_fn(s: String) -> String {
        lazy(|_| format!("a({})", s)).await
    }

    fn result_fn(s: String) -> Result<String, ()> {
        Ok(format!("r({})", s))
    }

    #[test]
    fn simple() {
        assert_eq!("foo".pipe(Some), Some("foo"));
    }

    #[test]
    fn chaining() {
        let a: Result<String, ()> = block_on(async {
            "foo"
                .to_string()
                .pipe(result_fn)?
                .pipe(|x| format!("c({})", x))
                .pipe(async_fn)
                .await
                .replace("f", "b")
                .pipe(Ok)
        });

        let b: Result<String, ()> = block_on(async {
            Ok(async_fn(format!("c({})", result_fn("foo".to_string())?))
                .await
                .replace("f", "b"))
        });

        assert_eq!(a, b);
        assert_eq!(a, Ok(String::from("a(c(r(boo)))")));
    }

    #[test]
    fn same_type() {
        let x: i32 = 3;
        let inc = |x| x + 1;
        let double = |x| x + x;
        let square = |x| x * x;
        let a = (x).pipe(inc).pipe(double).pipe(square);
        let b = square(double(inc(x)));
        assert_eq!(a, b);
    }

    #[test]
    fn type_transformation() {
        let x = 'x';
        let a = x.pipe(|x| (x, x, x)).pipe(|x| [x, x]);
        let b = [('x', 'x', 'x'), ('x', 'x', 'x')];
        assert_eq!(a, b);
    }

    #[test]
    fn slice() {
        let vec: &[i32] = &[0, 1, 2, 3];
        let vec = vec.pipe(|x: &[i32]| [x, &[4, 5, 6]].concat());
        assert_eq!(vec, [0, 1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn trait_object() {
        use core::{cmp::PartialEq, fmt::Display, marker::Copy};
        fn run(x: impl AsRef<str> + PartialEq + Display + Copy + ?Sized) {
            let x = x.pipe(|x| x);
            assert_eq!(x.as_ref(), "abc");
        }
        run("abc");
    }

    #[test]
    #[allow(clippy::blacklisted_name)]
    fn pipe_ref() {
        #[derive(Debug, PartialEq, Eq)]
        struct FooBar(i32);
        let foo = FooBar(12);
        let bar = foo.pipe_ref(|x| x.0).pipe(FooBar);
        assert_eq!(foo, bar);
    }

    #[test]
    #[allow(clippy::blacklisted_name)]
    fn pipe_ref_lifetime_bound() {
        #[derive(Debug, PartialEq, Eq)]
        struct Foo;
        fn f(foo: &'_ Foo) -> &'_ Foo {
            foo
        }
        Foo.pipe_ref(f).pipe_ref(f);
    }

    #[test]
    #[allow(clippy::blacklisted_name)]
    fn pipe_mut() {
        #[derive(Debug, PartialEq, Eq)]
        struct Foo(i32);
        let mut foo = Foo(0);
        foo.pipe_mut(|x| x.0 = 32);
        assert_eq!(foo, Foo(32));
    }

    #[test]
    #[allow(clippy::blacklisted_name)]
    fn pipe_mut_lifetime_bound() {
        #[derive(Debug, PartialEq, Eq)]
        struct Foo(i32, i32, i32);
        impl Foo {
            pub fn new() -> Self {
                Self(0, 0, 0)
            }
            pub fn set_0(&mut self, x: i32) -> &mut Self {
                self.0 = x;
                self
            }
            pub fn set_1(&mut self, x: i32) -> &mut Self {
                self.1 = x;
                self
            }
            pub fn set_2(&mut self, x: i32) -> &mut Self {
                self.2 = x;
                self
            }
        }

        let mut expected = Foo::new();
        let expected = expected.set_0(123).set_1(456).set_2(789);

        fn modify(foo: &mut Foo) -> &mut Foo {
            foo.set_0(123).set_1(456).set_2(789);
            foo
        }
        let mut actual = Foo::new();
        let actual = actual.pipe_mut(modify);

        assert_eq!(actual, expected);
    }
}
