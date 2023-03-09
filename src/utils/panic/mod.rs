//! The panic handler better behaviour support.
//!

mod handler;
pub use self::handler::*;

mod provider;
pub use self::provider::*;

mod validate;
pub use self::validate::*;

pub mod backtrace {
    //! The color backtrace support.
    //!
    pub use color_backtrace::*;
}
