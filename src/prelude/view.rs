use std::{fmt::Debug, rc::Rc};

use super::{Interest, Mediator, Notification, NotifyContext, Observer};

/// The definition for a PureMVC View.
///
/// In PureMVC, the [View] class assumes these responsibilities:
///
/// - Maintain a cache of [Mediator] instances.
/// - Provide methods for registering, retrieving, and removing [Mediator]'s.
/// - Managing the observer lists for each [Notification] in the application.
/// - Providing a method for attaching [Observer]'s to an [Notification]'s observer list.
/// - Providing a method for broadcasting an [Notification].
/// - Notifying the [Observer]'s of a given [Notification] when it broadcast.
///

pub trait View<Body>
where
    Body: Debug + 'static,
{
    /// Register an [Observer] to be notified of [Notification]'s with a given name.
    fn register_observer(&self, interest: Interest, observer: Rc<dyn Observer<Body>>);

    /// Remove a group of observers from the observer list for a given Notification name.
    fn remove_observer(&self, interest: &Interest, notify_context: &Rc<dyn NotifyContext>);

    /// Notify the [Observer]'s for a particular [Notification].
    ///
    /// All previously attached [Observer]'s for this [Notification]'s
    /// list are notified and are passed a reference to the [Notification] in
    /// the order in which they were registered.
    fn notify(&self, note: Rc<dyn Notification<Body>>);
}


/// Defines Mediator Registry functionality
pub trait MediatorRegistry<Body>
where
    Body: Debug + 'static,
{
    /// Register an [Mediator] instance with the [View].
    ///
    /// Registers the [Mediator] so that it can be retrieved by name,
    /// and further interrogates the [Mediator] for its [Notification] interests.
    ///
    /// If the [Mediator] returns any [Notification]
    /// names to be notified about, an [Observer] is created encapsulating
    /// the [Mediator] instance's [handle_notification](Mediator::handle_notification) method
    /// and registering it as an [Observer] for all [Notification]'s the
    /// [Mediator] is interested in.
    fn register_mediator<M: Mediator<Body>>(&self, mediator: Rc<M>);

    /// Retrieve an [Mediator] from the [View].
    fn retrieve_mediator<M: Mediator<Body>>(&self) -> Option<Rc<M>>;

    /// Remove an [Mediator] from the [View].
    fn remove_mediator<M: Mediator<Body>>(&self) -> Option<Rc<M>>;

    /// Check if a [Mediator] is registered or not
    fn has_mediator<M: Mediator<Body>>(&self) -> bool;
}
