#![allow(clippy::new_ret_no_self)]

//! A base INotification implementation.
//!
//! RuEx does not rely upon underlying event models such
//! as the one provided, does not have an inherent event model.
//!
//! The Observer Pattern as implemented within RuEx exists
//! to support event-driven communication between the
//! application and the actors of the MVC triad.
//!
//! Notifications are not meant to be a replacement for Events.
//! Generally, IMediator implementors
//! place event listeners on their view components, which they
//! then handle in the usual way. This may lead to the broadcast of Notifications to
//! trigger ICommands or to communicate with other IMediators. IProxy and ICommand
//! instances communicate with each other and IMediators
//! by broadcasting INotifications.
//!
//! A key difference between Events and RuEx
//! Notifications is that Events follow the
//! 'Chain of Responsibility' pattern, 'bubbling' up the display hierarchy
//! until some parent component handles the Event, while
//! RuEx Notifications follow a 'Publish/Subscribe'
//! pattern. RuEx classes need not be related to each other in a
//! parent/child relationship in order to communicate with one another
//! using Notifications.

#![allow(unused_variables)]
#![allow(dead_code)]

use crate::interfaces::*;

pub struct Notification<B> {
    /// the name of the notification instance
    name: String,
    /// the type of the notification instance
    ntype: String,
    /// the body of the notification instance
    body: B,
}

impl<B> Notification<B> {
    /// Constructor.
    ///
    /// * `name` - name of the Notification instance. (required)
    /// * `body` - the Notification body. (optional)
    /// * `type` - the type of the Notification (optional)
    /// body:B=null, type:String=null
    pub fn new(name: String, body: B, ntype: String) {
        // self.name = name;
        // self.body = body;
        // self.type = type;
    }
}

impl<B> INotification<B> for Notification<B> {
    /// Get the name of the Notification instance.
    ///
    /// Returns the name of the Notification instance.
    fn get_name(&self) -> String {
        // return self.name;
        unimplemented!()
    }

    /// Set the body of the Notification instance.
    fn set_body(&self, body: B) {
        // self.body = body;
        unimplemented!()
    }

    /// Get the body of the Notification instance.
    ///
    /// Returns the body object.
    fn get_body(&self) -> B {
        // self.body
        unimplemented!()
    }

    /// Set the type of the Notification instance.
    fn set_type(&self, ntype: String) {
        // self.type = type;
    }

    /// Get the type of the Notification instance.
    ///
    /// Returns the type
    fn get_type(&self) -> String {
        // self.ntype
        unimplemented!()
    }

    /// Get the string representation of the Notification instance.
    ///
    /// Returns the string representation of the Notification instance.
    fn to_string(&self) -> String {
        // var msg:String = "Notification Name: "+getName();
        // msg += "\nBody:"+(( body == null )?"null":body.toString());
        // msg += "\nType:"+(( type == null )?"null":type);
        // return msg;
        unimplemented!()
    }
}
