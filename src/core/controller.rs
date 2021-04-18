#![allow(clippy::new_ret_no_self)]

//! A Singleton IController implementation.
//!
//! In RuEx, the Controller class follows the
//! 'Command and Controller' strategy, and assumes these
//! responsibilities:
//!
//! *  Remembering which ICommands
//! are intended to handle which INotifications.
//! *  Registering itself as an IObserver with
//! the View for each INotification
//! that it has an ICommand mapping for.
//! *  Creating a new instance of the proper ICommand
//! to handle a given INotification when notified by the View.
//! *  Calling the ICommand's execute
//! method, passing in the INotification.
//!  
//! Your application must register ICommands with the
//! Controller.
//!
//! The simplest way is to subclass Facade,
//! and use its initializeController method to add your
//! registrations.
//!

#![allow(unused_variables)]
#![allow(dead_code)]

// use crate::core::*;
use crate::interfaces::*;
// use crate::patterns::observer::*;

// // Singleton instance
// protected static var instance : IController;

// // Message Constants
// protected const SINGLETON_MSG : String = "Controller Singleton already constructed!";

pub struct Controller<B, C, V> {
    /// Local reference to View
    view: Box<dyn IView<B, C, V>>,

    /// Mapping of Notification names to Command Class references
    command_map: Vec<String>,
}

impl<B, C, V> Controller<B, C, V> {
    /// Constructor.
    ///
    ///
    /// This IController implementation is a Singleton,
    /// so you should not call the constructor
    /// directly, but instead call the static Singleton
    /// Factory method Controller.getInstance()
    ///
    /// @throws Error Error if Singleton instance has already been constructed
    pub fn new() {
        // if (instance != null) throw Error(SINGLETON_MSG);
        // instance = this;
        // commandMap = new Array();
        // initializeController();
    }

    /// Initialize the Singleton Controller instance.
    ///
    /// Called automatically by the constructor.
    ///
    /// Note that if you are using a subclass of View
    /// in your application, you should also subclass Controller
    /// and override the initializeController method in the
    /// following way:
    ///
    /// //TODO: Example here
    ///
    fn initialize_controller() {
        // view = View.getInstance();
    }

    /// Controller Singleton Factory method.
    ///
    /// Returns the Singleton instance of Controller
    /// pub static fn
    pub fn get_instance() -> Box<dyn IController<B>> {
        // if ( instance == null ) instance = new Controller( );
        // return instance;
        unimplemented!()
    }
}

impl<B, C, V> IController<B> for Controller<B, C, V> {
    /// If an ICommand has previously been registered
    /// to handle a the given INotification, then it is executed.
    ///
    /// * `note` - an INotification
    fn execute_command(&self, note: Box<dyn INotification<B>>) {
        // var commandFactory : Box<dyn FnOnce()> = commandMap[ note.getName() ];
        // if ( commandFactory == null ) return;

        // var commandInstance : ICommand = new commandFactory();
        // commandInstance.execute( note );
    }

    /// Register a particular ICommand class as the handler
    /// for a particular INotification.
    ///
    /// If an ICommand has already been registered to
    /// handle INotifications with this name, it is no longer
    /// used, the new ICommand is used instead.
    ///
    /// The Observer for the new ICommand is only created if this the
    /// first time an ICommand has been regisered for this Notification name.
    ///
    /// * `notification_name` - the name of the INotification
    /// * `command_factory` - the Class of the ICommand
    fn register_command(&self, notification_name: String, command_factory: Box<dyn FnOnce()>) {
        // if ( commandMap[ notificationName ] == null ) {
        // 	view.registerObserver( notificationName, new Observer( executeCommand, this ) );
        // }
        // commandMap[ notificationName ] = commandFactory;
    }

    /// Check if a Command is registered for a given Notification
    ///
    /// * `notification_name` -
    /// Returns whether a Command is currently registered for the given notificationName.
    fn has_command(&self, notification_name: String) -> bool {
        // return commandMap[ notificationName ] != null;
        unimplemented!()
    }

    /// Remove a previously registered ICommand to INotification mapping.
    ///
    /// * `notification_name` - the name of the INotification to remove the ICommand mapping for
    fn remove_command(&self, notification_name: String) {
        // // if the Command is registered...
        // if ( hasCommand( notificationName ) )
        // {
        // 	// remove the observer
        // 	view.removeObserver( notificationName, this );

        // 	// remove the command
        // 	commandMap[ notificationName ] = null;
        // }
    }
}
