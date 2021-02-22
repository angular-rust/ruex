//! The interface definition for a RuEx Model.
//!
//! In RuEx, IModel implementors provide
//! access to IProxy objects by named lookup.
//!
//! An IModel assumes these responsibilities:
//!
//! * Maintain a cache of IProxy instances
//! * Provide methods for registering, retrieving, and removing IProxy instances

use super::proxy::IProxy;

pub trait IModel<D> {
    /// Register an IProxy instance with the Model.
    ///
    /// * `proxy_name` - the name to associate with this IProxy instance.
    /// * `proxy` - an object reference to be held by the Model.
    fn register_proxy(&self, proxy: Box<dyn IProxy<D>>);

    /// Retrieve an IProxy instance from the Model.
    ///
    /// * `proxy_name` -
    /// Returns the IProxy instance previously registered with the given proxy_name.
    fn retrieve_proxy(&self, proxy_name: String) -> Box<dyn IProxy<D>>;

    /// Remove an IProxy instance from the Model.
    ///
    /// * `proxy_name` - name of the IProxy instance to be removed.
    /// Returns the IProxy that was removed from the Model
    fn remove_proxy(&self, proxy_name: String) -> Box<dyn IProxy<D>>;

    /// Check if a Proxy is registered
    ///
    /// * `proxy_name` -
    /// Returns whether a Proxy is currently registered with the given proxy_name.
    fn has_proxy(&self, proxy_name: String) -> bool;
}
