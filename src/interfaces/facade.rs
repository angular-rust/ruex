//! The interface definition for a RuEx Facade.
//!
//! The Facade Pattern suggests providing a single
//! class to act as a central point of communication
//! for a subsystem.
//!
//! In RuEx, the Facade acts as an interface between
//! the core MVC actors (Model, View, Controller) and
//! the rest of your application.

use super::mediator::IMediator;
use super::notification::INotification;
use super::proxy::IProxy;
use super::notifier::INotifier;


pub trait IFacade<B, D, V>: INotifier<B> {
    /// Register an IProxy with the Model by name.
    ///
    /// * `proxy` - the IProxy to be registered with the Model.
    fn register_proxy(&self, proxy: Box<dyn IProxy<D>>);

    /// Retrieve a IProxy from the Model by name.
    ///
    /// * `proxy_name` - the name of the IProxy instance to be retrieved.
    /// Returns the IProxy previously regisetered by proxy_name with the Model.
    fn retrieve_proxy(&self, proxy_name: String) -> Box<dyn IProxy<D>>;

    /// Remove an IProxy instance from the Model by name.
    ///
    /// * `proxy_name` - the IProxy to remove from the Model.
    /// Returns the IProxy that was removed from the Model
    fn remove_proxy(&self, proxy_name: String) -> Box<dyn IProxy<D>>;

    /// Check if a Proxy is registered
    ///
    /// * `proxy_name` -
    /// Returns whether a Proxy is currently registered with the given proxy_name.
    fn has_proxy(&self, proxy_name: String) -> bool;

    /// Register an ICommand with the Controller.
    ///
    /// * `note_name` - the name of the INotification to associate the ICommand with.
    /// * `command_factory a reference to the Class of the ICommand.
    fn register_command(&self, note_name: String, command_factory: Box<dyn FnOnce()>);

    /// Remove a previously registered ICommand to INotification mapping from the Controller.
    ///
    /// * `notification_name` - the name of the INotification to remove the ICommand mapping for
    fn remove_command(&self, notification_name: String);

    /// Check if a Command is registered for a given Notification
    ///
    /// * `notification_name` -
    /// Returns whether a Command is currently registered for the given notification_name.
    fn has_command(&self, notification_name: String) -> bool;

    /// Register an IMediator instance with the View.
    ///
    /// * `mediator` - a reference to the IMediator instance
    fn register_mediator(&self, mediator: Box<dyn IMediator<B, V>>);

    /// Retrieve an IMediator instance from the View.
    ///
    /// * `mediator_name` - the name of the IMediator instance to retrievve
    /// Returns the IMediator previously registered with the given mediator_name.
    fn retrieve_mediator(&self, mediator_name: String) -> Box<dyn IMediator<B, V>>;

    /// Remove a IMediator instance from the View.
    ///
    /// * `mediator_name` - name of the IMediator instance to be removed.
    /// Returns the IMediator instance previously registered with the given mediator_name.
    fn remove_mediator(&self, mediator_name: String) -> Box<dyn IMediator<B, V>>;

    /// Check if a Mediator is registered or not
    ///
    /// * `mediator_name` -
    /// Returns whether a Mediator is registered with the given mediator_name.
    fn has_mediator(&self, mediator_name: String) -> bool;

    /// Notify the IObservers for a particular INotification.
    ///
    /// All previously attached IObservers for this INotification's
    /// list are notified and are passed a reference to the INotification in
    /// the order in which they were registered.
    ///
    /// NOTE: Use this method only if you are sending custom Notifications. Otherwise
    /// use the sendNotification method which does not require you to create the
    /// Notification instance.
    ///
    /// * `notification` - the INotification to notify IObservers of.
    fn notify_observers(&self, note: Box<dyn INotification<B>>);
}
