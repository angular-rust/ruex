//! PureMVC and other design patterns
//!
//! ## Model & Proxies
//! 
//! The [Model][2] simply caches named references to Proxies. Proxy code
//! manipulates the data model, communicating with remote services if
//! need be to persist or retrieve it.
//! 
//! This results in portable Model tier code.
//! 
//! ## View & Mediators
//! 
//! The View primarily caches named references to [Mediators][7]. [Mediator][7]
//! code stewards View Components, adding event listeners, sending
//! and receiving notifications to and from the rest of the system on
//! their behalf and directly manipulating their state.
//! 
//! This separates the View definition from the logic that controls it.
//! 
//! ## Controller & Commands
//! 
//! The [Controller][1] maintains named mappings to Command classes,
//! which are stateless, and only created when needed.
//! 
//! [Commands][9] may retrieve and interact with Proxies, send
//! Notifications, execute other [Commands][9], and are often used to
//! orchestrate complex or system-wide activities such as application
//! startup and shutdown. They are the home of your application’s
//! Business Logic.
//! 
//! ## Facade & Core
//! 
//! The [Facade][4], another Singleton, initializes the Core actors ([Model][2],
//! [View][3] and [Controller][1]), and provides a single place to access all of
//! their public methods.
//! 
//! By extending the [Facade][4], your application gets all the benefits of
//! Core actors without having to import and work with them directly.
//! You will implement a concrete [Facade][4] for your application only once
//! and it is simply done.
//! 
//! [Proxies][6], [Mediators][7] and [Commands][9] may then use your application’s
//! concrete [Facade][4] in order to access and communicate with each
//! other.
//! 
//! ## Observers & Notifications
//! 
//! PureMVC applications may run in environments without access to
//! Event and EventDispatcher classes, so the framework
//! implements an [Observer][8] notification scheme for communication
//! between the Core MVC actors and other parts of the system in a
//! loosely-coupled way.
//! 
//! You need not be concerned about the details of the PureMVC
//! [Observer][8]/[Notification][5] implementation; it is internal to the
//! framework. You will use a simple method to send [Notifications][5] from
//! [Proxies][6], [Mediators][7], [Commands][9] and the Facade itself that doesn’t
//! even require you to create a [Notification][5] instance.
//! 
//! What next: [Catalog of patterns..][patterns]
//! 
//! [1]: crate::prelude::Controller
//! [2]: crate::prelude::Model
//! [3]: crate::prelude::View
//! [4]: crate::prelude::Facade
//! [5]: crate::prelude::Notification
//! [6]: crate::prelude::Proxy
//! [7]: crate::prelude::Mediator
//! [8]: crate::prelude::Observer
//! [9]: crate::prelude::Command
//! 
pub mod patterns;