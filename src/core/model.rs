//! A Singleton IModel implementation.
//!
//! In RuEx, the Model class provides
//! access to model objects (Proxies) by named lookup.
//!
//! The Model assumes these responsibilities:
//!
//! * Maintain a cache of IProxy instances.
//! * Provide methods for registering, retrieving, and removing
//! IProxy instances.
//!
//! Your application must register IProxy instances
//! with the Model. Typically, you use an
//! ICommand to create and register IProxy
//! instances once the Facade has initialized the Core
//! actors.

#![allow(dead_code)]
#![allow(unused_variables)]

use crate::interfaces::*;

// // Singleton instance
// protected static var instance : IModel;

// // Message Constants
// protected const SINGLETON_MSG	: String = "Model Singleton already constructed!";

pub struct Model<D> {
    /// Mapping of proxyNames to IProxy instances
    proxy_map: Vec<D>, // FIXME: adjust
}

impl<D> Model<D> {
    /// Constructor.
    ///
    ///
    /// This IModel implementation is a Singleton,
    /// so you should not call the constructor
    /// directly, but instead call the static Singleton
    /// Factory method Model.getInstance()
    ///
    /// @throws Error Error if Singleton instance has already been constructed
    pub fn new() {
        // if (instance != null) throw Error(SINGLETON_MSG);
        // instance = this;
        // proxyMap = new Array();
        // initializeModel();
    }

    /// Initialize the Singleton Model instance.
    ///
    /// Called automatically by the constructor, this
    /// is your opportunity to initialize the Singleton
    /// instance in your subclass without overriding the
    /// constructor.
    ///
    fn initialize_model() {}

    /// Model Singleton Factory method.
    ///
    /// Returns the Singleton instance
    /// static fn
    fn get_instance() -> Box<dyn IModel<D>> {
        // if (instance == null) instance = new Model( );
        // return instance;
        unimplemented!()
    }
}

impl<D> IModel<D> for Model<D> {
    /// Register an IProxy with the Model.
    ///
    /// * `proxy` - an IProxy to be held by the Model.
    fn register_proxy(&self, proxy: Box<dyn IProxy<D>>) {
        // proxyMap[ proxy.getProxyName() ] = proxy;
        // proxy.onRegister();
    }

    /// Retrieve an IProxy from the Model.
    ///
    /// * `proxy_name` -
    /// Returns the IProxy instance previously registered with the given proxyName.
    fn retrieve_proxy(&self, proxy_name: String) -> Box<dyn IProxy<D>> {
        // return proxyMap[ proxyName ];
        unimplemented!()
    }

    /// Check if a Proxy is registered
    ///
    /// * `proxy_name` -
    /// Returns whether a Proxy is currently registered with the given proxyName.
    fn has_proxy(&self, proxy_name: String) -> bool {
        // return proxyMap[ proxy_name ] != null;
        unimplemented!()
    }

    /// Remove an IProxy from the Model.
    ///
    /// * `proxy_name` - name of the IProxy instance to be removed.
    /// Returns the IProxy that was removed from the Model
    fn remove_proxy(&self, proxy_name: String) -> Box<dyn IProxy<D>> {
        // var proxy:IProxy = proxyMap [ proxyName ] as IProxy;
        // if ( proxy )
        // {
        // 	proxyMap[ proxyName ] = null;
        // 	proxy.onRemove();
        // }
        // return proxy;
        unimplemented!()
    }
}
