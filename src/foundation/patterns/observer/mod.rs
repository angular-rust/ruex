//! Observers & Notifications
//!
//! PureMVC applications may run in environments without access to
//! Flash’s Event and EventDispatcher classes, so the framework
//! implements an Observer notification scheme for communication
//! between the Core MVC actors and other parts of the system in a
//! loosely-coupled way.
//! 
//! You need not be concerned about the details of the PureMVC
//! Observer/Notification implementation; it is internal to the
//! framework. You will use a simple method to send Notifications from
//! Proxies, Mediators, Commands and the Facade itself that doesn’t
//! even require you to create a Notification instance.
//! 
//! ## Notifications Can Be Used to Trigger Command Execution
//! 
//! Commands are mapped to Notification names in your concrete
//! Facade, and are automatically executed by the Controller when
//! their mapped Notifications are sent. Commands typically
//! orchestrate complex interaction between the interests of the View
//! and Model while knowing as little about each as possible.

mod notification;
pub use self::notification::*;

mod notifier;
pub use self::notifier::*;

mod observer;
pub use self::observer::*;
