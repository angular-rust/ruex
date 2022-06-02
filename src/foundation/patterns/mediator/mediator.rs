use std::{fmt, rc::Rc};

use crate::{
    foundation::patterns::facade::BaseFacade,
    prelude::{Facade, Interest, Mediator, Notification, Notifier, NotifyContext, Singleton, View},
};

/// A base [Mediator] implementation.

pub struct BaseMediator<Body> {
    // The view component
    view_component: Option<Rc<dyn View<Body>>>,
}

impl<Body> BaseMediator<Body> {
    /// The name of the [Mediator].
    ///
    /// Typically, a [Mediator] will be written to serve
    /// one specific control or group controls and so,
    /// will not have a need to be dynamically named.

    /// Constructor.
    pub fn new(view_component: Option<Rc<dyn View<Body>>>) -> Self {
        Self { view_component }
    }
}

impl<Body> Mediator<Body> for BaseMediator<Body>
where
    Body: fmt::Debug + 'static,
{
    fn view_component(&self) -> Option<Rc<dyn View<Body>>> {
        self.view_component.as_ref().map(|c| c.clone())
    }

    fn handle_notification(&self, _notification: Rc<dyn Notification<Body>>) {}

    fn list_notification_interests(&self) -> &[Interest] {
        &[]
    }

    fn on_register(&self) {}

    fn on_remove(&self) {}

    fn set_view_component(&mut self, view_component: Option<Rc<dyn View<Body>>>) {
        self.view_component = view_component;
    }
}

impl<Body> NotifyContext for BaseMediator<Body>
where
    Body: fmt::Debug + 'static,
{
    fn id(&self) -> u64 {
        0x01
    }
}

impl<Body> Notifier<Body> for BaseMediator<Body>
where
    Body: fmt::Debug + 'static,
{
    fn send(&self, interest: Interest, body: Option<Body>) {
        log::error!("You should implement yourself Mediator");
        BaseFacade::<Body>::global().send(interest, body);
    }
}

impl<Body> fmt::Debug for BaseMediator<Body>
where
    Body: fmt::Debug + 'static,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EmployeesMediator")
            //  .field("x", &self.x)
            .finish()
    }
}
