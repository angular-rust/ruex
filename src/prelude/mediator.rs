use std::{any::Any, fmt::Debug, rc::Rc};

use super::{Interest, Notification, NotifyContext, View};

/// The interface definition for a PureMVC Mediator.
///
/// In PureMVC, [Mediator] implementors assume these responsibilities:
///
/// - Implement a common method which returns a list of all [Notification]s
/// the [Mediator] has interest in.
/// - Implement a common notification (callback) method.
///
/// Additionally, [Mediator]s typically:
///
/// - Act as an intermediary between one or more view components such as text boxes or
/// list controls, maintaining references and coordinating their behavior.
/// - In Flash-based apps, this is often the place where event listeners are
/// added to view components, and their handlers implemented.
/// - Respond to and generate [Notification]s, interacting with of
/// the rest of the PureMVC app.
///
/// When an [Mediator] is registered with the [View],
/// the [View] will call the [Mediator]'s
/// [list_notification_interests] method. The [Mediator] will
/// return an [Vec] of [Notification] names which
/// it wishes to be notified about.
///
/// The [View] will then create an [Observer] object
/// encapsulating that [Mediator]'s ([handleNotification]) method
/// and register it as an Observer for each [Notification] name returned by
/// [list_notification_interests].
pub trait Mediator<Body>: NotifyContext + Debug + Sized + Any
where
    Body: Debug + 'static,
{
    /// Get the [Mediator]'s view component.
    fn view_component(&self) -> Option<Rc<dyn View<Body>>>;

    /// Set the [Mediator]'s view component.
    fn set_view_component(&mut self, component: Option<Rc<dyn View<Body>>>);

    /// List [Notification] interests.
    fn list_notification_interests(&self) -> &[Interest];

    /// Handle an [Notification].
    fn handle_notification(&self, notification: Rc<dyn Notification<Body>>);

    /// Called by the [View] when the [Mediator] is registered
    fn on_register(&self);

    /// Called by the [View] when the [Mediator] is removed
    fn on_remove(&self);
}
