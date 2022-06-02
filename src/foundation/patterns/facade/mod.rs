//! The base Facade implementation
//!
//! The three Core actors of the MVC meta-pattern are represented in
//! PureMVC by the Model, View and Controller classes. To simplify the
//! process of application development, PureMVC employs the Facade
//! pattern.
//! 
//! The Facade brokers your requests to the Model, View and Controller,
//! so that your code does not need import those classes and you do not
//! need to work with them individually. The Facade class automatically
//! instantiates the Core MVC Singletons in its constructor.
//! 
//! Typically, the framework Facade will be sub-classed in your application
//! and used to initialize the Controller with Command mappings.
//! Preparation of the Model and View are then orchestrated by
//! Commands executed by the Controller.
//! 
//! ## What is a Concrete Facade?
//! Though the Core actors are complete, usable implementations, the
//! Facade provides an implementation that should be considered
//! abstract, in that you never instantiate it directly.
//! 
//! Instead, you subclass the framework Facade and add or override
//! some of its methods to make it useful in your application.
//! 
//! This concrete Facade is then used to access and notify the
//! Commands, Mediators and Proxies that do the actual work of the
//! system. By convention, it is named ‘ApplicationFacade’, but you
//! may call it whatever you like.
//! 
//! Generally, your application’s View hierarchy (display components)
//! will be created by whatever process your platform normally
//! employs. In Flex, an MXML application instantiates all its children or
//! a Flash movie creates all the objects on its Stage. Once the
//! application’s View hierarchy has been built, the PureMVC apparatus
//! is started and the Model and View regions are prepared for use.
//! 
//! Your concrete Facade is also used to facilitate the startup process in
//! a way that keeps the main application code from knowing much
//! about the PureMVC apparatus to which it will be connected. The
//! application merely passes a reference to itself to a ‘startup’ method
//! on your concrete Facade’s Singleton instance.

mod facade;
pub use self::facade::*;
