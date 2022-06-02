use std::fmt;

/// Represent [Notification]'s interest
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Interest(pub u64);

/// The definition for a PureMVC Notification.
///
/// PureMVC does not rely upon underlying event models such
/// as the one provided with Flash.
///
/// The Observer Pattern as implemented within PureMVC exists
/// to support event-driven communication between the
/// application and the actors of the MVC triad.
///
/// Notifications are not meant to be a replacement for Events.
/// Generally, [Mediator] implementors
/// place event listeners on their view components, which they
/// then handle in the usual way. This may lead to the broadcast of [Notification]'s to
/// trigger [Command]'s or to communicate with other [Mediator]'s. [Proxy] and [Command]
/// instances communicate with each other and [Mediator]'s
/// by broadcasting [Notification]'s.
///
/// A key difference between native event's and PureMVC
/// [Notification]'s is that event's follow the
/// 'Chain of Responsibility' pattern, 'bubbling' up the display hierarchy
/// until some parent component handles the event, while
/// PureMVC [Notification]'s follow a 'Publish/Subscribe'
/// pattern. PureMVC classes need not be related to each other in a
/// parent/child relationship in order to communicate with one another
/// using [Notification]'s.
///
/// Should implement fmt::Debug to get the string representation of
/// the [Notification] instance
///
/// [Mediator]: crate::prelude::Mediator
/// [Proxy]: crate::prelude::Proxy
/// [Command]: crate::prelude::Command

pub trait Notification<Body>: fmt::Debug
where
    Body: fmt::Debug + 'static,
{
    /// Get the interest of the [Notification] instance
    /// No setter, should be set by constructor only
    fn interest(&self) -> Interest;

    /// Set the body of the [Notification] instance
    fn set_body(&mut self, body: Option<Body>);

    /// Get the body of the [Notification] instance
    fn body(&self) -> Option<&Body>;
}
