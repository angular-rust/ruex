//! A Base INotifier implementation.
//!
//! MacroCommand, Command, Mediator and Proxy
//! all have a need to send Notifications.
//!
//! The INotifier interface provides a common method called
//! sendNotification that relieves implementation code of
//! the necessity to actually construct Notifications.
//!
//! The Notifier class, which all of the above mentioned classes
//! extend, provides an initialized reference to the Facade
//! Singleton, which is required for the convienience method
//! for sending Notifications, but also eases implementation as these
//! classes have frequent Facade interactions and usually require
//! access to the facade anyway.

#![allow(unused_variables)]
#![allow(dead_code)]

use crate::interfaces::*;
// use crate::patterns::facade::Facade;

pub struct Notifier;

impl<B> INotifier<B> for Notifier {
    /// Create and send an INotification.
    ///
    ///
    /// Keeps us from having to construct new INotification
    /// instances in our implementation code.
    /// * `notification_name` - the name of the notiification to send
    /// * `body` - the body of the notification (optional)
    /// * `type` - the type of the notification (optional)
    /// body:T=null, type:String=null
    fn send_notification(&self, notification_name: String, body: B, ntype: String) {
        // facade::sendNotification(notificationName, body, ntype);
    }

    // Local reference to the Facade Singleton
    // facade: IFacade = Facade.getInstance();
}
