use std::fmt;

use crate::prelude::{Interest, Notification};

/// A base [Notification] implementation.
///
/// PureMVC does not rely upon underlying event models such
/// as the one provided with Flash.
///
/// The Observer Pattern as implemented within PureMVC exists
/// to support event-driven communication between the
/// application and the actors of the MVC triad.
///
/// Notifications are not meant to be a replacement for Events.
/// Generally, [Mediator] implementors place event listeners on their view components, which they
/// then handle in the usual way. This may lead to the broadcast of [Notification]s to
/// trigger [Command]s or to communicate with other [Mediator]s. [Proxy] and [Command]
/// instances communicate with each other and [Mediator]s  by broadcasting [Notification]s.
///
/// A key difference between Flash [Event]s and PureMVC
/// [Notification]s is that [Event]s follow the
/// 'Chain of Responsibility' pattern, 'bubbling' up the display hierarchy
/// until some parent component handles the [Event], while
/// PureMVC [Notification]s follow a 'Publish/Subscribe'
/// pattern. PureMVC classes need not be related to each other in a
/// parent/child relationship in order to communicate with one another
/// using [Notification]s.

pub struct BaseNotification<Body>
where
    Body: fmt::Debug + 'static,
{
    // the type of the notification instance
    interest: Interest,

    // the body of the notification instance
    body: Option<Body>,
}

impl<Body> BaseNotification<Body>
where
    Body: fmt::Debug + 'static,
{
    /// Constructor.
    pub fn new(interest: Interest, body: Option<Body>) -> Self {
        Self { interest, body }
    }
}

impl<Body> Notification<Body> for BaseNotification<Body>
where
    Body: fmt::Debug + 'static,
{
    fn body(&self) -> Option<&Body> {
        self.body.as_ref()
    }

    fn interest(&self) -> Interest {
        self.interest
    }

    fn set_body(&mut self, body: Option<Body>) {
        self.body = body;
    }
}

impl<Body> fmt::Debug for BaseNotification<Body>
where
    Body: fmt::Debug + 'static,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BaseNotification")
            .field("interest", &self.interest)
            .field("body", &self.body)
            .finish()
    }
}
