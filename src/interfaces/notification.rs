//! The interface definition for a RuEx Notification.
//!
//! RuEx does not rely upon underlying event models such
//! as the one provided, does not have an inherent event model.
//!
//! The Observer Pattern as implemented within RuEx exists
//! to support event-driven communication between the
//! application and the actors of the MVC triad.
//!
//! Notifications are not meant to be a replacement for Events.
//! Generally, IMediator implementors
//! place event listeners on their view components, which they
//! then handle in the usual way. This may lead to the broadcast of Notifications to
//! trigger ICommands or to communicate with other IMediators. IProxy and ICommand
//! instances communicate with each other and IMediators
//! by broadcasting INotifications.
//!
//! A key difference between Events and RuEx
//! Notifications is that Events follow the
//! 'Chain of Responsibility' pattern, 'bubbling' up the display hierarchy
//! until some parent component handles the Event, while
//! RuEx Notifications follow a 'Publish/Subscribe'
//! pattern. RuEx classes need not be related to each other in a
//! parent/child relationship in order to communicate with one another
//! using Notifications.

pub trait INotification<B> {
    /// Get the name of the INotification instance.
    /// No setter, should be set by constructor only
    fn get_name(&self) -> String;

    /// Set the body of the INotification instance
    fn set_body(&self, body: B);

    /// Get the body of the INotification instance
    fn get_body(&self) -> B;

    /// Set the type of the INotification instance
    fn set_type(&self, ntype: String);

    /// Get the type of the INotification instance
    fn get_type(&self) -> String;

    /// Get the string representation of the INotification instance
    fn to_string(&self) -> String;
}
