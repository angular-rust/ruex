//! Functions with lots of parameters are considered bad style and
//! reduce readability (“what does the 5th parameter mean?”).
//! Consider grouping some parameters into a new type.

/// Currying for functions with 2 params
///
/// ```no_run
/// fn two_params(a1: &str, a2: &str) -> String {
///     format!("{a1}/{a2}")
/// }
///
/// let one = two_params.curry("/mnt");
/// println!("{}", one("home"));
/// ```
pub trait Curry<A, B> {
    /// The concrete type that `curry` returns.
    type Output;
    /// Curry this function, transforming it from
    ///
    /// `fn(A, B) -> R`
    /// to
    /// `fn(A) -> fn(B) -> R`
    fn curry(self, a: A) -> Self::Output;
}

impl<Func, A, B, Out> Curry<A, B> for Func
where
    Func: Fn(A, B) -> Out + 'static,
    A: Copy + 'static,
{
    type Output = Box<dyn Fn(B) -> Out>;

    fn curry(self, a: A) -> Self::Output {
        Box::new(move |b: B| self(a, b))
    }
}

// not working magic !!! )))
// impl<Func, A, B> FnOnce<(A,)> for Func
// where
//     Func: Curry<A, B>,
// {
//     type Output = <Func as Curry<A, B>>::Output;
//     extern "rust-call" fn call_once(self, args: (A,)) -> Self::Output {
//         self.curry(args.0)
//     }
// }

/// Currying for functions with 3 params.
///
/// See [Curry] for details.

pub trait Curry2<A, B, C> {
    /// The concrete type that `curry` returns.
    type Output;
    /// Curry this function, transforming it from
    ///
    /// `fn(A, B, C) -> R`
    /// to
    /// `fn(A) -> fn(B, C) -> R`
    fn curry(self, a: A) -> Self::Output;
}

impl<Func, A, B, C, Out> Curry2<A, B, C> for Func
where
    Func: Fn(A, B, C) -> Out + 'static,
    A: Copy + 'static,
{
    type Output = Box<dyn Fn(B, C) -> Out>;

    fn curry(self, a: A) -> Self::Output {
        Box::new(move |b: B, c: C| self(a, b, c))
    }
}

/// Currying for functions with 4 params
///
/// See [Curry] for details.
pub trait Curry3<A, B, C, D> {
    /// The concrete type that `curry` returns.
    type Output;
    /// Curry this function, transforming it from
    ///
    /// `fn(A, B, C, D) -> R`
    /// to
    /// `fn(A) -> fn(B, C, D) -> R`
    fn curry(self, a: A) -> Self::Output;
}

impl<Func, A, B, C, D, Out> Curry3<A, B, C, D> for Func
where
    Func: Fn(A, B, C, D) -> Out + 'static,
    A: Copy + 'static,
{
    type Output = Box<dyn Fn(B, C, D) -> Out>;

    fn curry(self, a: A) -> Self::Output {
        Box::new(move |b: B, c: C, d: D| self(a, b, c, d))
    }
}

/// Currying for functions with 5 params
///
/// See [Curry] for details.
pub trait Curry4<A, B, C, D, E> {
    /// The concrete type that `curry` returns.
    type Output;
    /// Curry this function, transforming it from
    ///
    /// `fn(A, B, C, D, E) -> R`
    /// to
    /// `fn(A) -> fn(B, C, D, E) -> R`
    fn curry(self, a: A) -> Self::Output;
}

impl<Func, A, B, C, D, E, Out> Curry4<A, B, C, D, E> for Func
where
    Func: Fn(A, B, C, D, E) -> Out + 'static,
    A: Copy + 'static,
{
    type Output = Box<dyn Fn(B, C, D, E) -> Out>;

    fn curry(self, a: A) -> Self::Output {
        Box::new(move |b: B, c: C, d: D, e: E| self(a, b, c, d, e))
    }
}
