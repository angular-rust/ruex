//! The interface definition for a RuEx Notifier.
//!
//! MacroCommand, Command, Mediator and Proxy
//! all have a need to send Notifications.
//!
//! The INotifier interface provides a common method called
//! sendNotification that relieves implementation code of
//! the necessity to actually construct Notifications.
//!
//! The Notifier class, which all of the above mentioned classes
//! extend, also provides an initialized reference to the Facade
//! Singleton, which is required for the convienience method
//! for sending Notifications, but also eases implementation as these
//! classes have frequent Facade interactions and usually require
//! access to the facade anyway.

pub trait INotifier<B> {
    /// Send a INotification.
    ///
    ///
    /// Convenience method to prevent having to construct new
    /// notification instances in our implementation code.
    ///
    /// * `notification_name` - the name of the notification to send
    /// * `body` - the body of the notification (optional)
    /// * `type` - the type of the notification (optional)
    fn send_notification(&self, notification_name: String, body: B, ntype: String);
}
