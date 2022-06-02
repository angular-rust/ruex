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

mod controller;
pub use self::controller::*;

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

mod observer;
pub use self::observer::*;

mod proxy;
pub use self::proxy::*;

mod singleton;
pub use self::singleton::*;

mod view;
pub use self::view::*;
