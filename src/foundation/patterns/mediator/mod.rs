//! The base Proxy implementation
//!
//! A Mediator class is used to mediate the user's interaction with one or
//! more of the application's View Components and the rest of the PureMVC application.
//! 
//! In a application, a Mediator typically places event listeners
//! on its View Component to handle user gestures and requests for data
//! from the Component. It sends and receives Notifications to
//! communicate with the rest of the application.
//! 
//! ## Responsibilities of the Concrete Mediator
//! 
//! The Flash, Flex and AIR frameworks provide a vast array of richly-
//! interactive UI components. You may extend these or write your own
//! in ActionScript to provide endless possibilities for presenting the
//! data model to the user and allowing them to interact with it.
//! 
//! In the not so distant future, there will be other platforms running
//! ActionScript. And the framework has been ported and demonstrated
//! on other platforms already including Silverlight and J2ME, further
//! widening the horizons for RIA development with this technology.
//! 
//! A goal of the PureMVC framework is to be neutral to the
//! technologies being used at the boundaries of the application and
//! provide simple idioms for adapting whatever UI component or Data
//! structure/service you might find yourself concerned with at the
//! moment.
//!
//! To the PureMVC-based application, a View Component is any UI
//! component, regardless of what framework it is provided by or how
//! many sub-components it may contain. A View Component should
//! encapsulate as much of its own state and operation as possible,
//! exposing a simple API of events, methods and properties.
//! 
//! A concrete Mediator helps us adapt one or more View Components
//! to the application by holding the only references to those
//! components and interacting with the API they expose.
//! 
//! The responsibilities for the Mediator are primarily handling Events
//! dispatched from the View Component and relevant Notifications
//! sent from the rest of the system.
//! 
//! Since Mediators will also frequently interact with Proxies, it is
//! common for a Mediator to retrieve and maintain a local reference to
//! frequently accessed Proxies in its constructor. This reduces
//! repetitive retrieveProxy calls to obtain the same reference.

mod mediator;
pub use self::mediator::*;
