//! The interface definition for a RuEx Proxy.
//!
//! In RuEx, IProxy implementors assume these responsibilities:
//!
//! * Implement a common method which returns the name of the Proxy.
//! * Provide methods for setting and getting the data object.
//!
//! Additionally, IProxys typically:
//!
//! * Maintain references to one or more pieces of model data.
//! * Provide methods for manipulating that data.
//! * Generate INotifications when their model data changes.
//! * Expose their name as a public static const called NAME, if they are not instantiated multiple times.
//! * Encapsulate interaction with local or remote services used to fetch and persist model data.

pub trait IProxy<D> {
    /// Get the Proxy name
    ///
    /// Returns the Proxy instance name
    fn get_proxy_name(&self) -> String;

    /// Set the data object
    ///
    /// * `data` - the data object
    fn set_data(&self, data: D);

    /// Get the data object
    ///
    /// Returns the data as type D
    fn get_data(&self) -> D;

    /// Called by the Model when the Proxy is registered
    fn on_register(&self);

    /// Called by the Model when the Proxy is removed
    fn on_remove(&self);
}
