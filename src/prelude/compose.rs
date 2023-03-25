/// Function composition
///
/// Allows to compose functions and closures.
///
pub trait Compose<In, Out>: Fn(In) -> Out {
    /// `impl Trait` only allowed in function and inherent method return types,
    /// not in trait method return - so its boxed
    fn chain<Ret>(self, next: impl Fn(Out) -> Ret + 'static) -> Box<dyn Fn(In) -> Ret>;
}

impl<F, In, Out> Compose<In, Out> for F
where
    F: Fn(In) -> Out + 'static,
{
    fn chain<Ret>(self, next: impl Fn(Out) -> Ret + 'static) -> Box<dyn Fn(In) -> Ret> {
        Box::new(move |args: In| {
            // chained
            next(self(args))
        })
    }
}
