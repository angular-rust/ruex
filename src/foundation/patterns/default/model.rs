use once_cell::sync::OnceCell;
use std::{
    any::{Any, TypeId},
    cell::RefCell,
    collections::BTreeMap,
    rc::Rc,
};

use crate::prelude::{Model, Proxy, Singleton};

/// A Singleton [Model] implementation.
///
/// In PureMVC, the [Model] class provides access to model objects (Proxies) by named lookup.
///
/// The [Model] assumes these responsibilities:
///
/// - Maintain a cache of [Proxy] instances.
/// - Provide methods for registering, retrieving, and removing [Proxy] instances.
///
///
/// Your application must register [Proxy] instances with the [Model]. Typically, you use an
/// [Command] to create and register [Proxy] instances once the [Facade] has initialized the Core
/// actors.
pub struct BaseModel {
    // Mapping of proxy types to [Proxy] instances
    storages: RefCell<BTreeMap<TypeId, Rc<dyn Any>>>,
}

unsafe impl std::marker::Send for BaseModel {}
unsafe impl std::marker::Sync for BaseModel {}

impl BaseModel {
    // Create instance of BaseModel.
    //
    // This [Model] implementation is a Singleton, so you should not call the constructor
    // directly, but instead call the static Singleton Factory method [Model::instance()]
    pub fn new() -> Self {
        Self {
            storages: RefCell::new(BTreeMap::new()),
        }
    }
}

impl Singleton for BaseModel {
    /// Model Singleton Factory method
    ///
    fn global() -> &'static Self {
        static BASE_MODEL_INSTANCE: OnceCell<BaseModel> = OnceCell::new();
        BASE_MODEL_INSTANCE.get_or_init(Self::new)
    }
}

impl Model for BaseModel {
    fn has_proxy<P: Proxy>(&self) -> bool {
        let type_id = TypeId::of::<P>();
        self.storages.borrow().contains_key(&type_id)
    }

    fn register_proxy<P: Proxy>(&self, proxy: Rc<P>) {
        let type_id = TypeId::of::<P>();

        log::info!("Register Proxy [BaseModel] {:?}", proxy);

        self.storages.borrow_mut().insert(type_id, proxy.clone());

        proxy.on_register();
    }

    fn remove_proxy<P: Proxy>(&self) -> Option<Rc<P>> {
        let type_id = TypeId::of::<P>();

        self.storages
            .borrow_mut()
            .remove(&type_id)
            .map(|proxy| match proxy.downcast::<P>() {
                Ok(proxy) => {
                    proxy.on_remove();
                    proxy
                }
                Err(_) => {
                    panic!("Something wrong with proxy storage");
                }
            })
    }

    fn retrieve_proxy<P: Proxy>(&self) -> Option<Rc<P>> {
        // log::info!("Retrieve Proxy [BaseModel]");

        let type_id = TypeId::of::<P>();

        match self.storages.borrow().get(&type_id) {
            Some(item) => match item.clone().downcast::<P>() {
                Ok(proxy) => Some(proxy),
                Err(_) => {
                    log::error!("Something wrong with proxy storage");
                    None
                }
            },
            None => None,
        }
    }
}
