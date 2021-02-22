//! A base IProxy implementation.
//!
//! In RuEx, Proxy classes are used to manage parts of the
//! application's data model.
//!
//! A Proxy might simply manage a reference to a local data object,
//! in which case interacting with it might involve setting and
//! getting of its data in synchronous fashion.
//!
//! Proxy classes are also used to encapsulate the application's
//! interaction with remote services to save or retrieve data, in which case,
//! we adopt an asyncronous idiom; setting data (or calling a method) on the
//! Proxy and listening for a Notification to be sent
//! when the Proxy has retrieved the data from the service.

#![allow(unused_variables)]
#![allow(dead_code)]

use crate::interfaces::*;

const NAME: &str = "Proxy";

pub struct Proxy<D> {
    // the proxy name
    proxy_name: String,

    // the data object
    data: D,
}

impl<D> Proxy<D> {
    /// Constructor
    /// proxy_name:String=null, data:D=null
    pub fn new(proxy_name: String, data: D) {
        // self.proxy_name = (proxy_name != null)?NAME;
        // if data != null {
        // 	setData(data);
        // }
    }
}

impl<D> IProxy<D> for Proxy<D> {
    /// Get the proxy name
    fn get_proxy_name(&self) -> String {
        // self.proxyName
        unimplemented!()
    }

    /// Set the data object
    fn set_data(&self, data: D) {
        // self.data = data;
        unimplemented!()
    }

    /// Get the data object
    fn get_data(&self) -> D {
        // return self.data;
        unimplemented!()
    }

    /// Called by the Model when the Proxy is registered
    fn on_register(&self) {}

    /// Called by the Model when the Proxy is removed
    fn on_remove(&self) {}
}

// impl INotifier for Proxy {
// }
