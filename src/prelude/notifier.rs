use std::fmt::Debug;

use super::Interest;

/// The definition for a PureMVC Notifier.
///
/// [MacroCommand, Command, Mediator] and [Proxy]
/// all have a need to send [Notification]'s.
///
/// The [Notifier] interface provides a common method called
/// [send] that relieves implementation code of
/// the necessity to actually construct [Notification]'s.
///
/// The [Notifier] class, which all of the above mentioned classes
/// extend, also provides an initialized reference to the [Facade]
/// Singleton, which is required for the convienience method
/// for sending [Notification]'s, but also eases implementation as these
/// classes have frequent [Facade] interactions and usually require
/// access to the facade anyway.
/// 
/// [Notification]: crate::prelude::Notification
/// [Facade]: crate::prelude::Facade
/// [MacroCommand]: crate::prelude::MacroCommand
/// [Command]: crate::prelude::Command
/// [Mediator]: crate::prelude::Mediator
/// [Proxy]: crate::prelude::Proxy
/// [send]: Notifier::send

pub trait Notifier<Body>
where
    Body: Debug + 'static,
{
    /// Send a [Notification].
    ///
    /// Convenience method to prevent having to construct new
    /// notification instances in our implementation code.
    /// 
    /// [Notification]: crate::prelude::Notification
    fn send(&self, interest: Interest, body: Option<Body>);
}
