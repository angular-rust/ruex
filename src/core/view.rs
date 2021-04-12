#![allow(clippy::new_ret_no_self)]

//! A Singleton IView implementation.
//!
//! In RuEx, the View class assumes these responsibilities:
//!
//! * Maintain a cache of IMediator instances.
//! * Provide methods for registering, retrieving, and removing IMediators.
//! * Notifiying IMediators when they are registered or removed.
//! * Managing the observer lists for each INotification in the application.
//! * Providing a method for attaching IObservers to an INotification's observer list.
//! * Providing a method for broadcasting an INotification.
//! * Notifying the IObservers of a given INotification when it broadcast.
//!

#![allow(dead_code)]
#![allow(unused_variables)]

use crate::interfaces::*;
// use crate::patterns::observer::Observer;

// // Singleton instance
// protected static var instance	: IView;

// // Message Constants
// protected const SINGLETON_MSG	: String = "View Singleton already constructed!";

pub struct View<B, C, V> {
    /// Mapping of Mediator names to Mediator instances
    mediator_map: Vec<B>, // FIXME: adjust

    /// Mapping of Notification names to Observer lists
    observer_map: Vec<C>, // FIXME: adjust
    tmp: V, // FIXME: remove
}

impl<B, C, V> View<B, C, V> {
    /// Constructor.
    ///
    ///
    /// This IView implementation is a Singleton,
    /// so you should not call the constructor
    /// directly, but instead call the static Singleton
    /// Factory method View.getInstance()
    ///
    /// @throws Error Error if Singleton instance has already been constructed
    pub fn new() {
        // if (instance != null) throw Error(SINGLETON_MSG);
        // instance = this;
        // mediatorMap = new Array();
        // observerMap = new Array();
        // initializeView();
    }

    /// Initialize the Singleton View instance.
    ///
    /// Called automatically by the constructor, this
    /// is your opportunity to initialize the Singleton
    /// instance in your subclass without overriding the
    /// constructor.
    fn initialize_view() {}

    /// View Singleton Factory method.
    ///
    /// Returns the Singleton instance of View
    /// static
    pub fn get_instance() -> Box<dyn IView<B, C, V>> {
        // if ( instance == null ) instance = new View( );
        // return instance;
        unimplemented!()
    }
}

impl<B, C, V> IView<B, C, V> for View<B, C, V> {
    /// Register an IObserver to be notified
    /// of INotifications with a given name.
    ///
    /// * `notification_name` - the name of the INotifications to notify this IObserver of
    /// * `observer` - the IObserver to register
    fn register_observer(&self, notification_name: String, observer: Box<dyn IObserver<B, C>>) {
        // var observers:Array = observerMap[ notificationName ];
        // if( observers ) {
        // 	observers.push( observer );
        // } else {
        // 	observerMap[ notificationName ] = [ observer ];
        // }
    }

    /// Notify the IObservers for a particular INotification.
    ///
    /// All previously attached IObservers for this INotification's
    /// list are notified and are passed a reference to the INotification in
    /// the order in which they were registered.
    ///
    /// * `notification` - the INotification to notify IObservers of.
    fn notify_observers(&self, notification: Box<dyn INotification<B>>) {
        // if( observerMap[ notification.getName() ] != null ) {

        // 	// Get a reference to the observers list for this notification name
        // 	var observers_ref:Array = observerMap[ notification.getName() ] as Array;

        // 	// Copy observers from reference array to working array,
        // 	// since the reference array may change during the notification loop
        // 	var observers:Array = new Array();
        // 	var observer:IObserver;
        // 	for (var i:Number = 0; i < observers_ref.length; i++) {
        // 		observer = observers_ref[ i ] as IObserver;
        // 		observers.push( observer );
        // 	}

        // 	// Notify Observers from the working array
        // 	for (i = 0; i < observers.length; i++) {
        // 		observer = observers[ i ] as IObserver;
        // 		observer.notifyObserver( notification );
        // 	}
        // }
    }

    /// Remove the observer for a given notifyContext from an observer list for a given Notification name.
    ///
    /// * `notification_name` - which observer list to remove from
    /// * `notify_context` - remove the observer with this object as its notifyContext
    fn remove_observer(&self, notification_name: String, notify_context: C) {
        // // the observer list for the notification under inspection
        // var observers:Array = observerMap[ notificationName ] as Array;

        // // find the observer for the notifyContext
        // for ( var i:int=0; i<observers.length; i++ )
        // {
        // 	if ( Observer(observers[i]).compareNotifyContext( notifyContext ) == true ) {
        // 		// there can only be one Observer for a given notifyContext
        // 		// in any given Observer list, so remove it and break
        // 		observers.splice(i,1);
        // 		break;
        // 	}
        // }

        // // Also, when a Notification's Observer list length falls to
        // // zero, delete the notification key from the observer map
        // if ( observers.length == 0 ) {
        // 	delete observerMap[ notificationName ];
        // }
    }

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
    /// * `mediatorName` - the name to associate with this IMediator instance
    /// * `mediator` - a reference to the IMediator instance
    fn register_mediator(&self, mediator: Box<dyn IMediator<B, V>>) {
        // // do not allow re-registration (you must to removeMediator fist)
        // if ( mediatorMap[ mediator.getMediatorName() ] != null ) return;

        // // Register the Mediator for retrieval by name
        // mediatorMap[ mediator.getMediatorName() ] = mediator;

        // // Get Notification interests, if any.
        // var interests:Array = mediator.listNotificationInterests();

        // // Register Mediator as an observer for each of its notification interests
        // if ( interests.length > 0 )
        // {
        // 	// Create Observer referencing this mediator's handlNotification method
        // 	var observer:Observer = new Observer( mediator.handleNotification, mediator );

        // 	// Register Mediator as Observer for its list of Notification interests
        // 	for ( var i:Number=0;  i<interests.length; i++ ) {
        // 		registerObserver( interests[i],  observer );
        // 	}
        // }

        // // alert the mediator that it has been registered
        // mediator.onRegister();
    }

    /// Retrieve an IMediator from the View.
    ///
    /// * `mediator_name` - the name of the IMediator instance to retrieve.
    /// Returns the IMediator instance previously registered with the given mediatorName.
    fn retrieve_mediator(&self, mediator_name: String) -> Box<dyn IMediator<B, V>> {
        // return mediatorMap[ mediatorName ];
        unimplemented!()
    }

    /// Remove an IMediator from the View.
    ///
    /// * `mediator_name` - name of the IMediator instance to be removed.
    /// Returns the IMediator that was removed from the View
    fn remove_mediator(&self, mediator_name: String) -> Box<dyn IMediator<B, V>> {
        // // Retrieve the named mediator
        // var mediator:IMediator = mediatorMap[ mediatorName ] as IMediator;

        // if ( mediator )
        // {
        // 	// for every notification this mediator is interested in...
        // 	var interests:Array = mediator.listNotificationInterests();
        // 	for ( var i:Number=0; i<interests.length; i++ )
        // 	{
        // 		// remove the observer linking the mediator
        // 		// to the notification interest
        // 		removeObserver( interests[i], mediator );
        // 	}

        // 	// remove the mediator from the map
        // 	delete mediatorMap[ mediatorName ];

        // 	// alert the mediator that it has been removed
        // 	mediator.onRemove();
        // }

        // return mediator;
        unimplemented!()
    }

    /// Check if a Mediator is registered or not
    ///
    /// * `mediator_name` -
    /// Returns whether a Mediator is registered with the given mediatorName.
    fn has_mediator(&self, mediator_name: String) -> bool {
        // return mediatorMap[ mediatorName ] != null;
        unimplemented!()
    }
}
