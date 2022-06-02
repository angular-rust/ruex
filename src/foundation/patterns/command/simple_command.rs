use std::{fmt, rc::Rc};

use crate::{
    foundation::patterns::facade::BaseFacade,
    prelude::{Command, Facade, Interest, Notification, Notifier, Singleton},
};

/// A reference [Command] implementation.
pub struct SimpleCommand {}

impl SimpleCommand {}

impl<Body> Command<Body> for SimpleCommand
where
    Body: fmt::Debug + 'static,
{
    /// Fulfill the use-case initiated by the given [Notification].
    ///
    /// In the Command Pattern, an application use-case typically
    /// begins with some user action, which results in an [Notification] being broadcast, which
    /// is handled by business logic in the [execute](Command::execute) method of an [Command].
    fn execute(&self, _notification: Rc<dyn Notification<Body>>) {}
}

impl<Body> Notifier<Body> for SimpleCommand
where
    Body: fmt::Debug + 'static,
{
    fn send(&self, interest: Interest, body: Option<Body>) {
        log::error!("You should implement yourself SimpleCommand");
        BaseFacade::<Body>::global().send(interest, body);
    }
}

impl fmt::Debug for SimpleCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SimpleCommand").finish()
    }
}
