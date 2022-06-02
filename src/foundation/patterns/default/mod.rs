//! [Controller][1], [Model][2], [View][3] base implementation
//!
//! The PureMVC framework has a very narrow goal. That is to help you
//! separate your application’s coding concerns into three discrete tiers;
//! [Model][2], [View][3] and [Controller][1].
//! 
//! In this implementation of the classic MVC design meta-pattern, the
//! application tiers are represented by three Singletons (a class where
//! only one instance may be created).
//! 
//! A fourth Singleton, the Facade, simplifies development by providing a
//! single interface for communications throughout the application.
//! 
//! - The [Model][2] caches named references to [Proxies][6], which expose an API for
//! manipulating the Data Model (including data retrieved from remote services).
//! - The [View][3] primarily caches named references to [Mediator][4]'s, which adapt and
//! steward the [View][3] Components that make up the user interface.
//! - The [Controller][1] maintains named mappings to [Command][5] classes, which are
//! stateless, and only created when needed.
//! - The Facade initializes and caches the Core actors ([Model][2], [View][3] and
//! [Controller][1]), and provides a single place to access all of their public methods.
//!
//! [1]: crate::prelude::Controller
//! [2]: crate::prelude::Model
//! [3]: crate::prelude::View
//! [4]: crate::prelude::Mediator
//! [5]: crate::prelude::Command
//! [6]: crate::prelude::Proxy
//! 
mod controller;
pub use self::controller::*;

mod model;
pub use self::model::*;

mod view;
pub use self::view::*;
