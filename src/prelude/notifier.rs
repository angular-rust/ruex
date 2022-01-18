use std::fmt::Debug;

use super::Interest;

/// The interface definition for a PureMVC Notifier.
///
/// [MacroCommand, Command, Mediator] and [Proxy]
/// all have a need to send [Notification]s.
///
/// The [Notifier] interface provides a common method called
/// [sendNotification] that relieves implementation code of
/// the necessity to actually construct [Notification]s.
///
/// The [Notifier] class, which all of the above mentioned classes
/// extend, also provides an initialized reference to the [Facade]
/// Singleton, which is required for the convienience method
/// for sending [Notification]s, but also eases implementation as these
/// classes have frequent [Facade] interactions and usually require
/// access to the facade anyway.

pub trait Notifier<Body>
where
    Body: Debug + 'static,
{
    /// Send a [Notification].
    ///
    /// Convenience method to prevent having to construct new
    /// notification instances in our implementation code.
    fn send(&self, interest: Interest, body: Option<Body>);
}
