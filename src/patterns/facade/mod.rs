#![allow(clippy::new_ret_no_self)]

//! A base Singleton IFacade implementation.
//!
//! In RuEx, the Facade class assumes these
//! responsibilities:
//!
//! * Initializing the Model, View
//! and Controller Singletons.
//! * Providing all the methods defined by the IModel,
//! IView, & IController interfaces.
//! * Providing the ability to override the specific Model,
//! View and Controller Singletons created.
//! * Providing a single point of contact to the application for
//! registering Commands and notifying Observers
//!
//! //TODO: Example here

#![allow(unused_variables)]
#![allow(dead_code)]

use crate::interfaces::{
    IController, IFacade, IMediator, IModel, INotification, INotifier, IProxy, IView,
};

// // The Singleton Facade instance.
// protected static var instance : IFacade;

// // Message Constants
// protected const SINGLETON_MSG	: String = "Facade Singleton already constructed!";

pub struct Facade<B, C, D, V> {
    // Private references to Model, View and Controller
    controller: Box<dyn IController<B>>,
    model: Box<dyn IModel<D>>,
    view: Box<dyn IView<B, C, V>>,
}

impl<B, C, D, V> Facade<B, C, D, V> {
    /// Constructor.
    ///
    /// This IFacade implementation is a Singleton,
    /// so you should not call the constructor
    /// directly, but instead call the static Singleton
    /// Factory method Facade.getInstance()
    ///
    /// @throws Error Error if Singleton instance has already been constructed
    pub fn new() {
        // if (instance != null) throw Error(SINGLETON_MSG);
        // instance = this;
        // initializeFacade();
    }

    /// Initialize the Singleton Facade instance.
    ///
    /// Called automatically by the constructor. Override in your
    /// subclass to do any subclass specific initializations. Be
    /// sure to call super.initializeFacade(), though.
    fn initialize_facade() {
        // initializeModel();
        // initializeController();
        // initializeView();
    }

    /// Facade Singleton Factory method
    ///
    /// Returns the Singleton instance of the Facade
    /// static fn
    pub fn get_instance() -> Box<dyn IFacade<B, D, V>> {
        // if (instance == null) instance = new Facade( );
        // return instance;
        unimplemented!()
    }

    /// Initialize the Controller.
    ///
    /// Called by the initializeFacade method.
    /// Override this method in your subclass of Facade
    /// if one or both of the following are true:
    ///
    /// *  You wish to initialize a different IController.
    /// *  You have Commands to register with the Controller at startup..
    ///
    /// If you don't want to initialize a different IController,
    /// call super.initializeController() at the beginning of your
    /// method, then register Commands.
    fn initialize_controller() {
        // if ( controller != null ) return;
        // controller = Controller.getInstance();
    }

    /// Initialize the Model.
    ///
    /// Called by the initializeFacade method.
    /// Override this method in your subclass of Facade
    /// if one or both of the following are true:
    ///
    /// *  You wish to initialize a different IModel.
    /// *  You have Proxys to register with the Model that do not
    /// retrieve a reference to the Facade at construction time.
    ///
    /// If you don't want to initialize a different IModel,
    /// call super.initializeModel() at the beginning of your
    /// method, then register Proxys.
    ///
    /// Note: This method is rarely overridden; in practice you are more
    /// likely to use a Command to create and register Proxys
    /// with the Model, since Proxys with mutable data will likely
    /// need to send INotifications and thus will likely want to fetch a reference to
    /// the Facade during their construction.
    fn initialize_model() {
        // if ( model != null ) return;
        // model = Model.getInstance();
    }

    /// Initialize the View.
    ///
    /// Called by the initializeFacade method.
    /// Override this method in your subclass of Facade
    /// if one or both of the following are true:
    ///
    /// *  You wish to initialize a different IView.
    /// *  You have Observers to register with the View
    ///
    /// If you don't want to initialize a different IView,
    /// call super.initializeView() at the beginning of your
    /// method, then register IMediator instances.
    ///
    /// Note: This method is rarely overridden; in practice you are more
    /// likely to use a Command to create and register Mediators
    /// with the View, since IMediator instances will need to send
    /// INotifications and thus will likely want to fetch a reference
    /// to the Facade during their construction.
    fn initialize_view() {
        // if ( view != null ) return;
        // view = View.getInstance();
    }
}

impl<B, C, D, V> IFacade<B, D, V> for Facade<B, C, D, V> {
    /// Register an IProxy with the Model by name.
    ///
    /// * `proxyName` - the name of the IProxy.
    /// * `proxy` - the IProxy instance to be registered with the Model.
    fn register_proxy(&self, proxy: Box<dyn IProxy<D>>) {
        // model.registerProxy ( proxy );
    }

    /// Retrieve an IProxy from the Model by name.
    ///
    /// * `proxy_name` - the name of the proxy to be retrieved.
    /// Returns the IProxy instance previously registered with the given proxy_name.
    fn retrieve_proxy(&self, proxy_name: String) -> Box<dyn IProxy<D>> {
        // return model.retrieveProxy ( proxy_name );
        unimplemented!()
    }

    /// Remove an IProxy from the Model by name.
    ///
    /// * `proxy_name` - the IProxy to remove from the Model.
    /// Returns the IProxy that was removed from the Model
    fn remove_proxy(&self, proxy_name: String) -> Box<dyn IProxy<D>> {
        // var proxy:IProxy;
        // if ( model != null ) proxy = model.removeProxy ( proxy_name );
        // return proxy
        unimplemented!()
    }

    /// Check if a Proxy is registered
    ///
    /// * `proxy_name` -
    /// Returns whether a Proxy is currently registered with the given proxy_name.
    fn has_proxy(&self, proxy_name: String) -> bool {
        // return model.hasProxy( proxyName );
        unimplemented!()
    }

    /// Register an ICommand with the Controller by Notification name.
    ///
    /// * `notification_name` - the name of the INotification to associate the ICommand with
    /// * `command_factory` - a reference to the Class of the ICommand
    fn register_command(&self, notification_name: String, command_factory: Box<dyn FnOnce()>) {
        // controller.registerCommand( notificationName, commandFactory );
    }

    /// Remove a previously registered ICommand to INotification mapping from the Controller.
    ///
    /// * `notification_name` - the name of the INotification to remove the ICommand mapping for
    fn remove_command(&self, notification_name: String) {
        // controller.removeCommand( notification_name );
    }

    /// Check if a Command is registered for a given Notification
    ///
    /// * `notification_name` -
    /// Returns whether a Command is currently registered for the given notification_name.
    fn has_command(&self, notification_name: String) -> bool {
        // return controller.hasCommand(notification_name);
        unimplemented!()
    }

    /// Register a IMediator with the View.
    ///
    /// * `mediator` - a reference to the IMediator
    fn register_mediator(&self, mediator: Box<dyn IMediator<B, V>>) {
        // if ( view != null ) view.registerMediator( mediator );
    }

    /// Retrieve an IMediator from the View.
    ///
    /// * `mediator_name` -
    /// Returns the IMediator previously registered with the given mediatorName.
    fn retrieve_mediator(&self, mediator_name: String) -> Box<dyn IMediator<B, V>> {
        // return view.retrieveMediator( mediatorName ) as IMediator;
        unimplemented!()
    }

    /// Remove an IMediator from the View.
    ///
    /// * `mediator_name` - name of the IMediator to be removed.
    /// Returns the IMediator that was removed from the View
    fn remove_mediator(&self, mediator_name: String) -> Box<dyn IMediator<B, V>> {
        // var mediator:IMediator;
        // if ( view != null ) mediator = view.removeMediator( mediatorName );
        // return mediator;
        unimplemented!()
    }

    /// Check if a Mediator is registered or not
    ///
    /// * `mediator_name` -
    /// Returns whether a Mediator is registered with the given mediatorName.
    fn has_mediator(&self, mediator_name: String) -> bool {
        // return view.hasMediator( mediatorName );
        unimplemented!()
    }

    /// Notify Observers.
    ///
    /// This method is left public mostly for backward
    /// compatibility, and to allow you to send custom
    /// notification classes using the facade.
    ///
    /// Usually you should just call sendNotification
    /// and pass the parameters, never having to
    /// construct the notification yourself.
    ///
    /// * `notification` - the INotification to have the View notify Observers of.
    fn notify_observers(&self, notification: Box<dyn INotification<B>>) {
        // if view != null {
        //     view.notifyObservers(notification);
        // }
    }
}

impl<B, C, D, V> INotifier<B> for Facade<B, C, D, V> {
    /// Create and send an INotification.
    ///
    /// Keeps us from having to construct new notification
    /// instances in our implementation code.
    /// * `notification_name` - the name of the notiification to send
    /// * `body` - the body of the notification (optional)
    /// * `type` - the type of the notification (optional)
    /// body:T=null, type:String=null
    fn send_notification(&self, notification_name: String, body: B, ntype: String) {
        // notifyObservers( new Notification( notificationName, body, ntype ) );
    }
}
