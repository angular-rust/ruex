//! The interface definition for a RuEx Controller.
//!
//! In RuEx, an IController implementor
//! follows the 'Command and Controller' strategy, and
//! assumes these responsibilities:
//!
//! * Remembering which ICommands
//! are intended to handle which INotifications.
//! * Registering itself as an IObserver with
//! the View for each INotification
//! that it has an ICommand mapping for.
//! * Creating a new instance of the proper ICommand
//! to handle a given INotification when notified by the View.
//! * Calling the ICommand's execute
//! method, passing in the INotification.

use super::notification::INotification;

pub trait IController<B> {
    /// Register a particular ICommand class as the handler
    /// for a particular INotification.
    ///
    /// * `notification_name` - the name of the INotification
    /// * `command_factory` - the Class of the ICommand
    // FIXME: command_factory
    fn register_command(&self, notification_name: String, command_factory: Box<dyn FnOnce()>);

    /// Execute the ICommand previously registered as the
    /// handler for INotifications with the given notification name.
    ///
    /// * `notification` - the INotification to execute the associated ICommand for
    fn execute_command(&self, notification: Box<dyn INotification<B>>);

    /// Remove a previously registered ICommand to INotification mapping.
    ///
    /// * `notification_name` - the name of the INotification to remove the ICommand mapping for
    fn remove_command(&self, notification_name: String);

    /// Check if a Command is registered for a given Notification
    ///
    /// * `notification_name` -
    /// Returns whether a Command is currently registered for the given notificationName.
    fn has_command(&self, notification_name: String) -> bool;
}
