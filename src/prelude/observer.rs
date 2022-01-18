use std::{fmt::Debug, rc::Rc};

use super::Notification;

pub trait NotifyContext: Debug {
    fn id(&self) -> u64;
}

/// The interface definition for a PureMVC Observer.
///
/// In PureMVC, [Observer] implementors assume these responsibilities:
///
/// - Encapsulate the notification (callback) method of the interested object.
/// - Encapsulate the notification context (this) of the interested object.
/// - Provide methods for setting the interested object' notification method and context.
/// - Provide a method for notifying the interested object.
///
///
/// PureMVC does not rely upon underlying event
/// models such as the one provided with Flash.
///
/// The Observer Pattern as implemented within
/// PureMVC exists to support event driven communication
/// between the application and the actors of the MVC triad.
///
///  An Observer is an object that encapsulates information
/// about an interested object with a notification method that
/// should be called when an [Notification] is broadcast. The Observer then
/// acts as a proxy for notifying the interested object.
///
/// Observers can receive [Notification]s by having their
/// [notify] method invoked, passing
/// in an object implementing the [Notification] interface, such
/// as a subclass of [Notification].

pub trait Observer<Body>: Debug
where
    Body: Debug + 'static,
{
    // Get the notification context.
    fn context(&self) -> &Rc<dyn NotifyContext>;

    /// Set the notification method.
    ///
    /// The notification method should take one parameter of type [Notification]
    fn set_method(&mut self, notify_method: Box<dyn Fn(Rc<dyn Notification<Body>>)>);

    /// Set the notification context.
    fn set_context(&mut self, notify_context: Rc<dyn NotifyContext>);

    /// Notify the interested object.
    fn notify(&self, notification: Rc<dyn Notification<Body>>);

    /// Compare the given object to the notificaiton context object.
    fn compare_context(&self, object: &Rc<dyn NotifyContext>) -> bool;
}
