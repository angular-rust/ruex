//! The interface definition for a RuEx View.
//!
//! In RuEx, IView implementors assume these responsibilities:
//!
//! In RuEx, the View class assumes these responsibilities:
//!
//! * Maintain a cache of IMediator instances.
//! * Provide methods for registering, retrieving, and removing IMediators.
//! * Managing the observer lists for each INotification in the application.
//! * Providing a method for attaching IObservers to an INotification's observer list.
//! * Providing a method for broadcasting an INotification.
//! * Notifying the IObservers of a given INotification when it broadcast.

use super::mediator::IMediator;
use super::notification::INotification;
use super::observer::IObserver;

pub trait IView<B, C, V> {
    /// Register an IObserver to be notified
    /// of INotifications with a given name.
    ///
    /// * `notification_name` - the name of the INotifications to notify this IObserver of
    /// * `observer` - the IObserver to register
    fn register_observer(&self, notification_name: String, observer: Box<dyn IObserver<B, C>>);

    /// Remove a group of observers from the observer list for a given Notification name.
    ///
    /// * `notification_name` - which observer list to remove from
    /// * `notify_context` - removed the observers with this object as their notifyContext
    fn remove_observer(&self, notification_name: String, notify_context: C);

    /// Notify the IObservers for a particular INotification.
    ///
    /// All previously attached IObservers for this INotification's
    /// list are notified and are passed a reference to the INotification in
    /// the order in which they were registered.
    ///
    /// * `notification` - the INotification to notify IObservers of.
    fn notify_observers(&self, note: Box<dyn INotification<B>>);

    /// Register an IMediator instance with the View.
    ///
    /// Registers the IMediator so that it can be retrieved by name,
    /// and further interrogates the IMediator for its
    /// INotification interests.
    ///
    /// If the IMediator returns any INotification
    /// names to be notified about, an Observer is created encapsulating
    /// the IMediator instance's handleNotification method
    /// and registering it as an Observer for all INotifications the
    /// IMediator is interested in.</p>
    ///
    /// * `mediator_name` - the name to associate with this IMediator instance
    /// * `mediator` - a reference to the IMediator instance
    fn register_mediator(&self, mediator: Box<dyn IMediator<B, V>>);

    /// Retrieve an IMediator from the View.
    ///
    /// * `mediator_name` - the name of the IMediator instance to retrieve.
    /// Returns the IMediator instance previously registered with the given mediator_name.
    fn retrieve_mediator(&self, mediator_name: String) -> Box<dyn IMediator<B, V>>;

    /// Remove an IMediator from the View.
    ///
    /// * `mediator_name` - name of the IMediator instance to be removed.
    /// Returns the IMediator that was removed from the View
    fn remove_mediator(&self, mediator_name: String) -> Box<dyn IMediator<B, V>>;

    /// Check if a Mediator is registered or not
    ///
    /// * `mediator_name` -
    /// Returns whether a Mediator is registered with the given mediator_name.
    fn has_mediator(&self, mediator_name: String) -> bool;
}
