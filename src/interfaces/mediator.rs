//! The interface definition for a RuEx Mediator.
//!
//! In RuEx, IMediator implementors assume these responsibilities:
//!
//! * Implement a common method which returns a list of all INotifications
//! the IMediator has interest in.
//! * Implement a notification callback method.
//! * Implement methods that are called when the IMediator is registered or removed from the View.
//!
//! Additionally, IMediators typically:
//!
//! * Act as an intermediary between one or more view components such as text boxes or
//! list controls, maintaining references and coordinating their behavior.
//! * This is often the place where event listeners are
//! added to view components, and their handlers implemented.
//! * Respond to and generate INotifications, interacting with of
//! the rest of the RuEx app.
//!
//!
//! When an IMediator is registered with the IView,
//! the IView will call the IMediator's
//! listNotificationInterests method. The IMediator will
//! return an Array of INotification names which
//! it wishes to be notified about.
//!
//! The IView will then create an Observer object
//! encapsulating that IMediator's (handleNotification) method
//! and register it as an Observer for each INotification name returned by
//! listNotificationInterests.
//!
//! //TODO: Example here

use super::notification::INotification;

pub trait IMediator<B, V> {
    /// Get the IMediator instance name
    ///
    /// Returns the IMediator instance name
    fn get_mediator_name(&self) -> String;

    /// Get the IMediator's view component.
    ///
    /// Returns the view component
    fn get_view_component(&self) -> V;

    /// Set the IMediator's view component.
    ///
    /// * `view_component` - the view component
    fn set_view_component(&self, view_component: V);

    /// List INotification interests.
    ///
    /// Returns an Array of the INotification names this IMediator has an interest in.
    fn list_notification_interests(&self) -> Vec<String>;

    /// Handle an INotification.
    ///
    /// * `notification` - the INotification to be handled
    fn handle_notification(&self, notification: Box<dyn INotification<B>>);

    /// Called by the View when the Mediator is registered
    fn on_register(&self);

    /// Called by the View when the Mediator is removed
    fn on_remove(&self);
}
