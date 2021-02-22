//! The interface definition for a RuEx Command.

use super::notification::INotification;

pub trait ICommand<B> {
    /// Execute the ICommand's logic to handle a given INotification.
    ///
    /// * `note` - an Notification to handle.
    fn execute(&self, notification: Box<dyn INotification<B>>);
}
