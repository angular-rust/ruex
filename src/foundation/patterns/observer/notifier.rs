use std::fmt::Debug;

use crate::{
    foundation::patterns::facade::BaseFacade,
    prelude::{Facade, Interest, Notifier, Singleton},
};

/// A Base [Notifier] implementation.
///
/// [MacroCommand], [Command], [Mediator] and [Proxy]
/// all have a need to send [Notification]s.
///
/// The [Notifier] interface provides a common method called
/// [sendNotification] that relieves implementation code of
/// the necessity to actually construct [Notification]s.
///
/// The [Notifier] class, which all of the above mentioned classes
/// extend, provides an initialized reference to the [Facade]
/// Singleton, which is required for the convienience method
/// for sending [Notification]s, but also eases implementation as these
/// classes have frequent [Facade] interactions and usually require
/// access to the facade anyway.

pub struct BaseNotifier;

impl BaseNotifier {
    pub fn new() -> Self {
        Self {}
    }
}

impl<Body> Notifier<Body> for BaseNotifier
where
    Body: Debug + 'static,
{
    fn send(&self, interest: Interest, body: Option<Body>) {
        log::error!("You should implement yourself Notifier");
        BaseFacade::<Body>::global().send(interest, body);
    }
}
