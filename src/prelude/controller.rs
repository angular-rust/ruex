use std::{fmt::Debug, rc::Rc};

use super::{Command, Interest, Notification};

/// The interface definition for a PureMVC Controller.
///
/// In PureMVC, an [Controller] implementor
/// follows the 'Command and Controller' strategy, and
/// assumes these responsibilities:
///
/// - Remembering which [Command]s
/// are intended to handle which [Notification]s
/// - Registering itself as an [Observer] with
/// the [View] for each [Notification]
/// that it has an [Command] mapping for
/// - Creating a new instance of the proper [Command]
/// to handle a given [Notification] when notified by the [View]
/// - Calling the [Command]'s [execute]
/// method, passing in the [Notification]
///

pub trait Controller<Body>
where
    Body: Debug + 'static,
{
    /// Register a particular [Command] class as the handler
    /// for a particular [Notification].
    fn register_command(&self, interest: Interest, command: Rc<dyn Command<Body>>);

    /// Execute the [Command] previously registered as the
    /// handler for [Notification]s with the given notification name.
    fn execute_command(&self, notification: Rc<dyn Notification<Body>>);

    /// Remove a previously registered [Command] to [Notification] mapping.
    fn remove_command(&self, interest: &Interest);

    /// Check if a Command is registered for a given Notification
    fn has_command(&self, interest: &Interest) -> bool;
}
