use std::rc::Rc;

use super::Proxy;

/// The interface definition for a PureMVC Model.
///
/// In PureMVC, [Model] implementors provide
/// access to [Proxy] objects by named lookup.
///
/// An [Model] assumes these responsibilities:
///
/// - Maintain a cache of [Proxy] instances
/// - Provide methods for registering, retrieving, and removing [Proxy] instances
///

pub trait Model {
    /// Register an [Proxy] instance with the [Model].
    fn register_proxy<P: Proxy>(&self, proxy: Rc<P>);

    /// Retrieve an [Proxy] instance from the Model.
    fn retrieve_proxy<P: Proxy>(&self) -> Option<Rc<P>>;

    /// Remove an [Proxy] instance from the Model.
    fn remove_proxy<P: Proxy>(&self) -> Option<Rc<P>>;

    /// Check if a [Proxy] is registered
    fn has_proxy<P: Proxy>(&self) -> bool;
}
