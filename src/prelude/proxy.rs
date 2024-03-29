use std::{any::Any, fmt::Debug};

/// The definition for a PureMVC Proxy.
///
/// In PureMVC, [Proxy] implementors assume these responsibilities:
///
/// - Implement a common method which returns the name of the [Proxy].
///
/// Additionally, [Proxy]'s typically:
///
/// - Maintain references to one or more pieces of model data.
/// - Provide methods for manipulating that data.
/// - Generate [Notification]'s when their model data changes.
/// - Expose their name using [Debug] or [Display].
/// - Encapsulate interaction with local or remote services used to fetch and persist model data.
///
/// [Notification]: crate::prelude::Notification
/// [Debug]: std::fmt::Debug
/// [Display]: std::fmt::Display

pub trait Proxy: Debug + Sized + Any {
    /// Called by the Model when the [Proxy] is registered
    fn on_register(&self);

    /// Called by the Model when the [Proxy] is removed
    fn on_remove(&self);
}
