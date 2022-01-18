use std::{fmt, rc::Rc};

use crate::prelude::{Notification, NotifyContext, Observer};

/// A base [Observer] implementation.
///
/// An [Observer] is an object that encapsulates information
/// about an interested object with a method that should
/// be called when a particular [Notification] is broadcast.
///
/// In PureMVC, the [Observer] class assumes these responsibilities:
///
/// - Encapsulate the notification (callback) method of the interested object.
/// - Encapsulate the notification context (this) of the interested object.
/// - Provide methods for setting the notification method and context.
/// - Provide a method for notifying the interested object.
///

pub struct BaseObserver<Body>
where
    Body: fmt::Debug + 'static,
{
    notify: Box<dyn Fn(Rc<dyn Notification<Body>>)>,
    context: Rc<dyn NotifyContext>,
}

impl<Body> BaseObserver<Body>
where
    Body: fmt::Debug + 'static,
{
    /// Constructor.
    ///
    /// The notification method on the interested object should take
    /// one parameter of type [Notification]
    pub fn new(notify: Box<dyn Fn(Rc<dyn Notification<Body>>)>, context: Rc<dyn NotifyContext>) -> Self {
        Self { notify, context }
    }

    // Get the notification method.
    fn method(&self) -> &impl Fn(Rc<dyn Notification<Body>>) {
        &self.notify
    }
}

impl<Body> Observer<Body> for BaseObserver<Body>
where
    Body: fmt::Debug + 'static,
{
    // Get the notification context.
    fn context(&self) -> &Rc<dyn NotifyContext> {
        &self.context
    }

    fn compare_context(&self, object: &Rc<dyn NotifyContext>) -> bool {
        object.id() == self.context.id()
    }

    fn notify(&self, notification: Rc<dyn Notification<Body>>) {
        self.method()(notification);
    }

    fn set_context(&mut self, context: Rc<dyn NotifyContext>) {
        self.context = context;
    }

    fn set_method(&mut self, notify: Box<dyn Fn(Rc<dyn Notification<Body>>)>) {
        self.notify = notify;
    }
}

impl<Body> fmt::Debug for BaseObserver<Body>
where
    Body: fmt::Debug + 'static,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BaseObserver").field("context", &self.context).finish()
    }
}
