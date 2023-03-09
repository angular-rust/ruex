use std::{
    any::{Any, TypeId},
    cell::RefCell,
    collections::{BTreeMap, HashMap},
    fmt,
    rc::Rc,
};

use crate::{
    foundation::patterns::observer::BaseObserver,
    prelude::{
        Interest, Mediator, MediatorRegistry, Notification, NotifyContext, Observer, Singleton,
        View,
    },
};

/// A Singleton [View] implementation.
///
/// In PureMVC, the [View] class assumes these responsibilities:
///
/// - Maintain a cache of [Mediator] instances.
/// - Provide methods for registering, retrieving, and removing [Mediator]'s.
/// - Managing the observer lists for each [Notification] in the application.
/// - Providing a method for attaching [Observer]'s to an [Notification]'s observer list.
/// - Providing a method for broadcasting an [Notification].
/// - Notifying the [Observer]'s of a given [Notification] when it broadcast.
///

pub struct BaseView<Body>
where
    Body: fmt::Debug + 'static,
{
    // Mapping of Mediator types to Mediator instances
    mediator_map: RefCell<BTreeMap<TypeId, Rc<dyn Any>>>,

    // Mapping of Notification names to Observer lists
    observer_map: RefCell<HashMap<Interest, Vec<Rc<dyn Observer<Body>>>>>,
}

unsafe impl<Body> std::marker::Send for BaseView<Body> where Body: fmt::Debug + 'static {}
unsafe impl<Body> std::marker::Sync for BaseView<Body> where Body: fmt::Debug + 'static {}

impl<Body> BaseView<Body>
where
    Body: fmt::Debug + 'static,
{
    /// Create instance of BaseView.
    ///
    /// Actually, you have to reimplement the [View] for your purposes with Singleton pattern.
    ///
    /// This [View] implementation is keept here in educational purposes only.
    ///
    pub fn new() -> Self {
        Self {
            mediator_map: RefCell::new(BTreeMap::new()),
            observer_map: RefCell::new(HashMap::new()),
        }
    }
}

impl<Body> Singleton for BaseView<Body>
where
    Body: fmt::Debug + 'static,
{
    /// View Singleton Factory method
    ///
    /// It is not possible to implement Singleton with generics.
    /// So you should implement it in your final code only.
    ///
    /// Error: use of generic parameter from outer function
    ///
    fn global() -> &'static Self {
        // static BASE_VIEW_INSTANCE: OnceCell<BaseFacade<Body>> = OnceCell::new();
        // BASE_VIEW_INSTANCE.get_or_init(Self::new)
        todo!("you have to reimplement the view for your purposes")
    }
}

impl<Body> View<Body> for BaseView<Body>
where
    Body: fmt::Debug + 'static,
{
    fn notify(&self, note: Rc<dyn Notification<Body>>) {
        // Copy observers from reference array to working array,
        // since the reference array may change during the notification loop
        // and prevent double borrow ))
        let observers = self.observer_map.borrow().get(&note.interest()).cloned();

        if let Some(observers) = observers {
            for observer in observers.iter() {
                log::info!("Notify observer {:?} for {:?}", observer, note.interest());
                observer.notify(note.clone());
            }
        }
    }

    fn register_observer(&self, interest: Interest, observer: Rc<dyn Observer<Body>>) {
        // log::info!("Register Observer [BaseView] {:?}", interest);
        let mut observer_map = self.observer_map.borrow_mut();

        observer_map.entry(interest).or_insert_with(Vec::new);

        if let Some(observers) = observer_map.get_mut(&interest) {
            observers.push(observer)
        }
    }

    // It private so its fun
    fn remove_observer(&self, interest: &Interest, context: &Rc<dyn NotifyContext>) {
        let mut observer_map = self.observer_map.borrow_mut();

        // the observer list for the notification under inspection
        if let Some(observers) = observer_map.remove(interest).as_mut() {
            // find the observer for the notify_context
            for (idx, observer) in observers.iter().enumerate() {
                if observer.compare_context(context) {
                    // there can only be one Observer for a given notify_context
                    // in any given Observer list, so remove it and break
                    observers.remove(idx);
                    break;
                }
            }
        }
    }
}

impl<Body> MediatorRegistry<Body> for BaseView<Body>
where
    Body: fmt::Debug + 'static,
{
    fn register_mediator<M: Mediator<Body>>(&self, mediator: Rc<M>) {
        log::info!("Register Mediator [BaseView] {:?}", mediator);

        let mut mediator_map = self.mediator_map.borrow_mut();

        let type_id = TypeId::of::<M>();
        // do not allow re-registration (you must to removeMediator fist)
        if mediator_map.contains_key(&type_id) {
            return;
        }

        // Register the Mediator for retrieval by name
        mediator_map.insert(type_id, mediator.clone());

        // Get Notification interests, if any.
        let interests = mediator.list_notification_interests();
        if !interests.is_empty() {
            let mediator = mediator.clone();
            let context = mediator.clone();
            // Create Observer
            let observer = Rc::new(BaseObserver::new(
                Box::new(move |notification| {
                    log::info!("Observer notify {:?}", notification);
                    mediator.handle_notification(notification.clone())
                }),
                context,
            ));

            // Register Mediator as Observer for its list of Notification interests
            for interest in interests.iter() {
                self.register_observer(*interest, observer.clone());
            }
        }

        mediator.on_register();
    }

    fn retrieve_mediator<M: Mediator<Body>>(&self) -> Option<Rc<M>> {
        let type_id = TypeId::of::<M>();

        match self.mediator_map.borrow().get(&type_id) {
            Some(item) => match item.clone().downcast::<M>() {
                Ok(mediator) => Some(mediator),
                Err(_) => {
                    log::error!("Something wrong with proxy storage");
                    None
                }
            },
            None => None,
        }
    }

    fn remove_mediator<M: Mediator<Body>>(&self) -> Option<Rc<M>> {
        // remove the mediator from the map
        let type_id = TypeId::of::<M>();

        self.mediator_map
            .borrow_mut()
            .remove(&type_id)
            .map(|mediator| {
                match mediator.downcast::<M>() {
                    Ok(mediator) => {
                        // for every notification this mediator is interested in...
                        let interests = mediator.list_notification_interests();
                        for interest in interests.iter() {
                            // remove the observer linking the mediator
                            // to the notification interest

                            let mut observer_map = self.observer_map.borrow_mut();

                            let context = mediator.id();

                            // the observer list for the notification under inspection
                            if let Some(observers) = observer_map.remove(interest).as_mut() {
                                // find the observer for the notify_context
                                for (idx, observer) in observers.iter().enumerate() {
                                    if observer.context().id() == context {
                                        // there can only be one Observer for a given notify_context
                                        // in any given Observer list, so remove it and break
                                        observers.remove(idx);
                                        break;
                                    }
                                }
                            }
                        }

                        // alert the mediator that it has been removed
                        mediator.on_remove();
                        mediator
                    }
                    Err(_) => {
                        panic!("Something wrong with mediator storage");
                    }
                }
            })
    }

    fn has_mediator<M: Mediator<Body>>(&self) -> bool {
        let type_id = TypeId::of::<M>();
        self.mediator_map.borrow().contains_key(&type_id)
    }
}
