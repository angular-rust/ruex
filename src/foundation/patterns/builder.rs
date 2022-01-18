use std::cell::RefCell;

use crate::prelude::{Getter, SetterMut, With, WithBuilder};

/// Promote the Getter trait.
impl<Target, From> Getter<From> for Target
where
    From: AsRef<Target>,
{
    fn get(from: &From) -> &Target {
        from.as_ref()
    }
}

/// Builder pattern implementation.
pub struct Builder<T: Default> {
    pub inner: RefCell<Option<T>>,
}

impl<T: Default> Default for Builder<T> {
    fn default() -> Self {
        Self {
            inner: RefCell::new(Some(Default::default())),
        }
    }
}

impl<T: Default> Builder<T> {
    pub fn build(&self) -> Option<T> {
        self.inner.borrow_mut().take()
    }
}

/// Promote With trait for all objects which implement Setter trait.
///
/// Current version of Rust is not supported trait exclusion from trait bounds.
///
/// So if you need IMP'ish version of With, you should to implement it yourself
/// something like a:
/// `impl With<Title> for Builder<Window>`
///
impl<Param, Target> With<Param> for Builder<Target>
where
    Target: Default + SetterMut<Param>,
{
    fn with(self, param: Param) -> Self {
        self.inner
            .borrow_mut()
            .as_mut()
            .map(|val| SetterMut::<Param>::set(val, param));
        self
    }
}

/// Promote the WithBuilder trait for all builders which able
/// to configure specific parameter types.
impl<Target, Param> WithBuilder<Param> for Target
where
    Builder<Target>: With<Param>,
    Target: Default,
{
    fn with(param: Param) -> Builder<Self> {
        Builder::<Self>::default().with(param)
    }
}

// /// Simple method to generate builder of object.
// pub trait Construction<T> {
//     fn construct() -> Builder<T>;
// }

// impl<T> Construction<T> for T
// where
//     T: Default + Clone,
// {
//     fn construct() -> Builder<T> {
//         Builder::<T>::default()
//     }
// }
