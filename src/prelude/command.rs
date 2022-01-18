use std::{fmt::Debug, rc::Rc};

use super::Notification;

/// The interface definition for a PureMVC Command.
pub trait Command<Body>: Debug
where
    Body: Debug + 'static,
{
    /// Execute the [Command]'s logic to handle a given [Notification].
    fn execute(&self, notification: Rc<dyn Notification<Body>>);
}
