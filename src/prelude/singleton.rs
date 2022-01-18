/// Singleton Factory method
///
pub trait Singleton {
    fn global() -> &'static Self;
}
