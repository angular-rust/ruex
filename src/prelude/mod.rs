//! The `ruex` prelude.
//!
//! The purpose of this module is to alleviate imports of many common ruex
//! traits by adding a glob import to the top of ruex heavy modules:
//!
//! ```
//! # #![allow(unused_imports)]
//! use ruex::prelude::*;
//! ```

mod builder;
pub use self::builder::*;

mod command;
pub use self::command::*;

mod compose;
pub use self::compose::*;

mod controller;
pub use self::controller::*;

mod currying;
pub use self::currying::*;

mod facade;
pub use self::facade::*;

mod mediator;
pub use self::mediator::*;

mod model;
pub use self::model::*;

mod notification;
pub use self::notification::*;

mod notifier;
pub use self::notifier::*;

// mod pipe;
// pub use self::pipe::*;

mod observer;
pub use self::observer::*;

mod proxy;
pub use self::proxy::*;

mod singleton;
pub use self::singleton::*;

mod view;
pub use self::view::*;

pub use crate::utils::*;

// pub use ruex_macro::aspect;
// pub use ruex_macro::decorate;
// pub use ruex_macro::delegate;
// pub use ruex_macro::register;
// pub use ruex_macro::Compose;
pub use ruex_macro::*;

/// Joint point stub for aspect development
pub struct AspectJointPoint;

impl AspectJointPoint {
    /// Marker for joint point
    pub fn proceed() {}
}
