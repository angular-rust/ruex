//! The interface definition for a RuEx Observer.
//!
//! In RuEx, IObserver implementors assume these responsibilities:
//!
//! * Encapsulate the notification (callback) method of the interested object.
//! * Encapsulate the notification context (this) of the interested object.
//! * Provide methods for setting the interested object' notification method and context.
//! * Provide a method for notifying the interested object.
//!
//! RuEx does not rely upon underlying event
//! models such as the one provided, does not have an inherent
//! event model.
//!
//! The Observer Pattern as implemented within
//! RuEx exists to support event driven communication
//! between the application and the actors of the
//! MVC triad.
//!
//! An Observer is an object that encapsulates information
//! about an interested object with a notification method that
//! should be called when an INotification is broadcast. The Observer then
//! acts as a proxy for notifying the interested object.
//!
//! Observers can receive Notifications by having their
//! notifyObserver method invoked, passing
//! in an object implementing the INotification interface, such
//! as a subclass of Notification.

use crate::interfaces::INotification;

pub trait IObserver<B, C> {
    /// Set the notification method.
    ///
    /// The notification method should take one parameter of type INotification
    ///
    /// * `notify_method` - the notification (callback) method of the interested object
    fn set_notify_method(&self, notify_method: Box<dyn FnOnce()>); // FIXME: 

    /// Set the notification context.
    ///
    /// * `notify_context` - the notification context (this) of the interested object
    fn set_notify_context(&self, notify_context: C);

    /// Notify the interested object.
    ///
    /// * `notification` - the INotification to pass to the interested object's notification method
    fn notify_observer(&self, notification: Box<dyn INotification<B>>);

    /// Compare the given object to the notificaiton context object.
    ///
    /// * `object` - the object to compare.
    /// Returns boolean indicating if the notification context and the object are the same.
    fn compare_notify_context(&self, object: C) -> bool;
}
