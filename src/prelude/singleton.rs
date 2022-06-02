/// Singleton Factory method
///
pub trait Singleton {
    /// Retrieve instance of Singleton
    fn global() -> &'static Self;
}
