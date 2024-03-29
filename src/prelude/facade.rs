use std::{fmt::Debug, rc::Rc};

use super::{Command, Interest};

/// The definition for a PureMVC Facade.
///
/// The Facade Pattern suggests providing a single
/// class to act as a central point of communication
/// for a subsystem.
///
/// In PureMVC, the Facade acts as an interface between
/// the core MVC actors [Model], [View], [Controller] and
/// the rest of your application.
///
/// Also Facade should implement IModel trait with Model
/// for different data types and IView
/// 
/// [Model]: crate::prelude::Model
/// [View]: crate::prelude::View
/// [Controller]: crate::prelude::Controller
pub trait Facade<Body>
where
    Body: Debug + 'static,
{
    /// Register an [Command] with the [Controller].
    /// 
    /// [Controller]: crate::prelude::Controller
    fn register_command(&self, interest: Interest, command: Rc<dyn Command<Body>>);

    /// Remove a previously registered [Command] to [Notification] mapping from the [Controller].
    /// 
    /// [Notification]: crate::prelude::Notification
    /// [Controller]: crate::prelude::Controller
    fn remove_command(&self, interest: &Interest);

    /// Check if a [Command] is registered for a given [Notification]
    /// 
    /// [Notification]: crate::prelude::Notification
    fn has_command(&self, interest: &Interest) -> bool;

    /// Create and send an [Notification].
    /// 
    /// [Notification]: crate::prelude::Notification
    fn send(&self, interest: Interest, body: Option<Body>);
}
