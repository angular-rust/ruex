use crate::foundation::patterns::builder::Builder;


/// Powerfull way to build simple objects.
pub trait With<T> {
    /// Set param
    fn with(self, param: T) -> Self;
}

/// Powerfull way to get object properties.
pub trait Getter<T> {
    fn get(from: &T) -> &Self;
}

/// Powerfull way to configure objects.
pub trait SetterMut<T> {
    /// Set param
    fn set(&mut self, param: T) -> &mut Self;
}

/// Powerfull way to configure objects.
pub trait Setter<T> {
    /// Set param
    fn set(&self, param: T) -> &Self;
}

/// Powerfull way to configure objects with builder pattern.
pub trait BuildWith<T> {
    /// Set param
    fn with(param: T) -> Self;
}

/// Trait which generate builder by set the some parameter.
pub trait WithBuilder<P>: Sized {
    fn with(param: P) -> Builder<Self>
    where
        Self: Default;
}
