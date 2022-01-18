use std::fmt;

use crate::{
    foundation::patterns::facade::BaseFacade,
    prelude::{Facade, Interest, Notifier, Proxy, Singleton},
};

/// A base [Proxy] implementation.
///
/// In PureMVC, [Proxy] classes are used to manage parts of the
/// application's data model.
///
/// A [Proxy] might simply manage a reference to a local data object,
/// in which case interacting with it might involve setting and
/// getting of its data in synchronous fashion.
///
/// [Proxy] classes are also used to encapsulate the application's
/// interaction with remote services to save or retrieve data, in which case,
/// we adopt an asyncronous idiom; setting data (or calling a method) on the
/// [Proxy] and listening for a [Notification] to be sent
/// when the [Proxy] has retrieved the data from the service.

pub struct BaseProxy<Body>
where
    Body: fmt::Debug + 'static,
{
    // the data object
    pub data: Option<Body>,
}

impl<Body> BaseProxy<Body>
where
    Body: fmt::Debug + 'static,
{
    /// Constructor
    pub fn new(data: Option<Body>) -> Self {
        Self { data }
    }
}

impl<Body> Proxy for BaseProxy<Body>
where
    Body: fmt::Debug + 'static,
{
    fn on_register(&self) {}

    fn on_remove(&self) {}
}

impl<Body> Notifier<Body> for BaseProxy<Body>
where
    Body: fmt::Debug + 'static,
{
    fn send(&self, interest: Interest, body: Option<Body>) {
        log::error!("You should implement yourself Proxy");
        BaseFacade::<Body>::global().send(interest, body);
    }
}

impl<Body> fmt::Debug for BaseProxy<Body>
where
    Body: fmt::Debug + 'static,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Proxy<Body>")
            //  .field("x", &self.x)
            .finish()
    }
}
