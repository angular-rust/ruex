//! The base Proxy implementation
//! 
//! Generally speaking, the Proxy pattern is used to provide a placeholder
//! for an object in order to control access to it. In a PureMVC-based
//! application, the Proxy class is used specifically to manage a portion of
//! the application's data model.
//! 
//! A Proxy might manage access to a locally created data structure of
//! arbitrary complexity. This is the Proxy’s Data Object.
//! 
//! In this case, idioms for interacting with it probably involve
//! synchronous setting and getting of its data. It may expose all or part
//! of its Data Object’s properties and methods, or a reference to the Data
//! Object itself. When exposing methods for updating the data, it may
//! also send Notifications to the rest of the system that the data has
//! changed.
//! 
//! A Remote Proxy might be used to encapsulate interaction with a
//! remote service to save or retrieve a piece of data. The Proxy can
//! maintain the object that communicates with the remote service, and
//! control access to the data sent and received from the service.
//! 
//! In such a case, one might set data or call a method of the Proxy and
//! await an asynchronous Notification, sent by the Proxy when the
//! service has received the data from the remote endpoint.
//! 
//! ## Responsibilities of the Concrete Proxy
//! 
//! The concrete Proxy allows us to encapsulate a piece of the data
//! model, wherever it comes from and whatever its type, by managing
//! the Data Object and the application’s access to it.
//! 
//! The Proxy implementation class that comes with PureMVC is a
//! simple data carrier object that can be registered with the Model.
//! 
//! Though it is completely usable in this form, you will usually subclass
//! Proxy and add functionality specific to the particular Proxy.
//! Common variations on the Proxy pattern include:
//! 
//! - Remote Proxy, where the data managed by the
//!   concrete Proxy is in a remote location and will be
//!   accessed via a service of some sort.
//!   
//! - Proxy and Delegate, where access to a service object
//!   needs to be shared between multiple Proxies. The
//!   Delegate class maintains the service object and
//!   controls access to it, ensuring that responses are
//!   properly routed to their requestors.
//!   
//! - Protection Proxy, used when objects need to have
//!   different access rights.
//!   o Virtual Proxy, which creates expensive objects on
//!   demand.
//!   
//! - Smart Proxy, loads data object into memory on first
//!   access, performs reference counting, allows locking
//!   of object to ensure no other object can change it.

mod proxy;
pub use self::proxy::*;
