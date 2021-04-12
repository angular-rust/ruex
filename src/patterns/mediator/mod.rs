#![allow(clippy::new_ret_no_self)]

//! A base IMediator implementation.
//!
//! The name of the Mediator.
//!
//! Typically, a Mediator will be written to serve
//! one specific control or group controls and so,
//! will not have a need to be dynamically named.

#![allow(unused_variables)]
#![allow(dead_code)]

use crate::interfaces::*;
// use crate::patterns::facade::Facade;
// use crate::patterns::observer::*;

const NAME: &str = "Mediator";

pub struct Mediator<V> {
    /// the mediator name
    mediator_name: String,

    /// The view component
    view_component: V,
}

impl<V> Mediator<V> {
    /// Constructor.
    /// mediatorName:String=null, viewComponent:V=null
    pub fn new(mediator_name: String, view_component: V) {
        // self.mediatorName = (mediator_name != null)?mediator_name:NAME;
        // self.viewComponent = viewComponent;
    }
}

impl<B, V> IMediator<B, V> for Mediator<V> {
    /// Get the name of the Mediator.
    /// Returns the Mediator name
    fn get_mediator_name(&self) -> String {
        // return mediator_name;
        unimplemented!()
    }

    /// Get the Mediator's view component.
    ///
    /// Additionally, an implicit getter will usually
    /// be defined in the subclass that casts the view
    /// object to a type, like this:
    ///
    /// //TODO: Example here
    ///
    /// Returns the view component
    fn get_view_component(&self) -> V {
        // return view_component;
        unimplemented!()
    }

    /// Set the IMediator's view component.
    ///
    /// * `view_component` - the view component
    fn set_view_component(&self, view_component: V) {
        // self.view_component = view_component;
    }

    /// List the INotification names this
    /// Mediator is interested in being notified of.
    ///
    /// Returns Array the list of INotification names
    fn list_notification_interests(&self) -> Vec<String> {
        // return [];
        unimplemented!()
    }

    /// Handle INotifications.
    ///
    /// Typically this will be handled in a switch statement,
    /// with one 'case' entry per INotification
    /// the Mediator is interested in.
    fn handle_notification(&self, notification: Box<dyn INotification<B>>) {}

    /// Called by the View when the Mediator is registered
    fn on_register(&self) {}

    /// Called by the View when the Mediator is removed
    fn on_remove(&self) {}
}

// impl INotifier for Mediator {
// }
