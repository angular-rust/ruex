//! A base IObserver implementation.
//!  
//! An Observer is an object that encapsulates information
//! about an interested object with a method that should
//! be called when a particular INotification is broadcast.
//!
//! In RuEx, the Observer class assumes these responsibilities:
//!
//! * Encapsulate the notification (callback) method of the interested object.
//! * Encapsulate the notification context (this) of the interested object.
//! * Provide methods for setting the notification method and context.
//! * Provide a method for notifying the interested object.

#![allow(unused_variables)]
#![allow(dead_code)]

use crate::interfaces::*;

pub struct Observer<C> {
    notify: Box<dyn FnOnce()>,
    context: C,
}

impl<C> Observer<C> {
    /// Constructor.
    ///
    /// The notification method on the interested object should take
    /// one parameter of type INotification
    ///
    /// * `notifyMethod` - the notification method of the interested object
    /// * `notifyContext` - the notification context of the interested object
    pub fn new(notify_method: Box<dyn FnOnce()>, notify_context: C) {
        // setNotifyMethod(notify_method);
        // setNotifyContext(notify_context);
    }

    /// Get the notification method.
    ///
    /// Returns the notification (callback) method of the interested object.
    fn get_notify_method(&self) -> Box<dyn FnOnce()> {
        // self.notify
        unimplemented!()
    }

    /// Get the notification context.
    ///
    /// Returns the notification context (this) of the interested object.
    fn get_notify_context(&self) -> C {
        // self.context
        unimplemented!()
    }
}

impl<B, C> IObserver<B, C> for Observer<C> {
    /// Set the notification method.
    ///
    /// The notification method should take one parameter of type INotification.
    ///
    /// * `notify_method` - the notification (callback) method of the interested object.
    fn set_notify_method(&self, notify_method: Box<dyn FnOnce()>) {
        // self.notify = notifyMethod;
    }

    /// Set the notification context.
    ///
    /// * `notify_context` - the notification context (this) of the interested object.
    fn set_notify_context(&self, notify_context: C) {
        // self.context = notifyContext;
    }

    /// Notify the interested object.
    ///
    /// * `notification` - the INotification to pass to the interested object's notification method.
    fn notify_observer(&self, notification: Box<dyn INotification<B>>) {
        // self.getNotifyMethod()
        //     .apply(self.getNotifyContext(), [notification]);
    }

    /// Compare an object to the notification context.
    ///
    /// * `object` - the object to compare
    /// Returns boolean indicating if the object and the notification context are the same
    fn compare_notify_context(&self, object: C) -> bool {
        // return object == self.context;
        unreachable!()
    }
}
