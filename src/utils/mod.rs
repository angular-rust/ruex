//! The `ruex` utilites.
//!
//! ```
//! # #![allow(unused_imports)]
//! use ruex::utils::*;
//! ```

pub mod terminal;

pub mod panic;

pub mod logging {
    //! Configurable logging support.
    //!
    pub use fern::*;

    pub use syslog;

    pub use log::LevelFilter;
}

pub use humantime::*;

pub mod build {
    //! Build script and macro support.
    //!
    pub use companion::*;
}
