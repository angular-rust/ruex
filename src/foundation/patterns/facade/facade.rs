// use once_cell::sync::OnceCell;
use std::{fmt::Debug, rc::Rc};

use crate::{
    foundation::patterns::{
        default::{BaseController, BaseModel, BaseView},
        observer::{BaseNotification, BaseObserver},
    },
    prelude::*,
};

/// A base Singleton [Facade] implementation.
///
/// In PureMVC, the [Facade] class assumes these
/// responsibilities:
///
/// - Initializing the [Model], [View] and [Controller] Singletons.
/// - Providing all the methods defined by the [Model], [View], & [Controller] interfaces.
/// - Providing a single point of contact to the application for registering [Command]s and notifying [Observer]s
///

pub struct BaseFacade<Body>
where
    Body: Debug + 'static,
{
    // Private references to Model, View and Controller
    controller: BaseController<Body>,
    view: BaseView<Body>,
}

impl<Body> BaseFacade<Body>
where
    Body: Debug + 'static,
{
    /// Create instance of BaseFacade.
    ///
    /// Actually, you have to reimplement the [Facade] for your purposes with Singleton pattern.
    ///
    /// This [Facade] implementation is keept here in educational purposes only.

    pub fn new() -> Self {
        Self {
            controller: BaseController::new(),
            view: BaseView::new(),
        }
    }
}

impl<Body> Singleton for BaseFacade<Body>
where
    Body: Debug + 'static,
{
    /// Facade Singleton Factory method
    ///
    /// It is not possible to implement Singleton with generics.
    /// So you should implement it in your final code only.
    ///
    /// Error: use of generic parameter from outer function
    ///
    fn global() -> &'static Self {
        // static BASE_FACADE_INSTANCE: OnceCell<BaseFacade<Body>> = OnceCell::new();
        // BASE_FACADE_INSTANCE.get_or_init(Self::new)
        todo!("you have to reimplement the facade for your purposes")
    }
}

impl<Body> Facade<Body> for BaseFacade<Body>
where
    Body: Debug + 'static,
{
    fn has_command(&self, interest: &Interest) -> bool {
        self.controller.has_command(interest)
    }

    fn register_command(&self, interest: Interest, command: Rc<dyn Command<Body>>) {
        {
            // this code should be located in [Controller] in your final version
            // check the [BaseController]::register_command

            if !self.has_command(&interest) {
                self.view.register_observer(
                    interest,
                    Rc::new(BaseObserver::new(
                        Box::new(|notification| {
                            log::error!("You should implement yourself Facade");
                            BaseFacade::<Body>::global()
                                .controller
                                .execute_command(notification);
                        }),
                        self.controller.as_context(),
                    )),
                );
            }
        }

        self.controller.register_command(interest, command);
    }

    fn remove_command(&self, interest: &Interest) {
        if self.has_command(interest) {
            {
                // this code should be located in [Controller] in your final version
                // check the [BaseController]::remove_command

                // remove the observer
                self.view
                    .remove_observer(interest, &self.controller.as_context());
            }

            self.controller.remove_command(interest);
        }
    }

    fn send(&self, interest: Interest, body: Option<Body>) {
        self.notify(Rc::new(BaseNotification::new(interest, body)));
    }
}

impl<Body> Model for BaseFacade<Body>
where
    Body: Debug + 'static,
{
    /// Check if a [Proxy] is registered
    fn has_proxy<P: Proxy>(&self) -> bool {
        BaseModel::global().has_proxy::<P>()
    }

    /// Register an [Proxy] with the [Model] by name.
    fn register_proxy<P: Proxy>(&self, proxy: Rc<P>) {
        BaseModel::global().register_proxy(proxy);
    }

    /// Remove an [Proxy] instance from the [Model] by name.
    fn remove_proxy<P: Proxy>(&self) -> Option<Rc<P>> {
        BaseModel::global().remove_proxy::<P>()
    }

    /// Retrieve a [Proxy] from the [Model] by name.
    fn retrieve_proxy<P: Proxy>(&self) -> Option<Rc<P>> {
        BaseModel::global().retrieve_proxy::<P>()
    }
}

impl<Body> View<Body> for BaseFacade<Body>
where
    Body: Debug + 'static,
{
    fn register_observer(&self, interest: Interest, observer: Rc<dyn Observer<Body>>) {
        self.view.register_observer(interest, observer);
    }

    fn remove_observer(&self, interest: &Interest, notify_context: &Rc<dyn NotifyContext>) {
        self.view.remove_observer(interest, notify_context);
    }

    /// Notify the [Observer]s for a particular [Notification].
    ///
    /// All previously attached [Observer]s for this [Notification]'s
    /// list are notified and are passed a reference to the [Notification] in
    /// the order in which they were registered.
    ///
    /// NOTE: Use this method only if you are sending custom Notifications. Otherwise
    /// use the sendNotification method which does not require you to create the
    /// Notification instance.
    fn notify(&self, note: Rc<dyn Notification<Body>>) {
        self.view.notify(note);
    }
}

impl<Body> MediatorRegistry<Body> for BaseFacade<Body>
where
    Body: Debug + 'static,
{
    /// Register an [Mediator] instance with the [View].
    fn register_mediator<M: Mediator<Body>>(&self, mediator: Rc<M>) {
        self.view.register_mediator(mediator);
    }

    /// Retrieve an [Mediator] instance from the [View].
    fn retrieve_mediator<M: Mediator<Body>>(&self) -> Option<Rc<M>> {
        self.view.retrieve_mediator::<M>()
    }

    /// Remove a [Mediator] instance from the [View].
    fn remove_mediator<M: Mediator<Body>>(&self) -> Option<Rc<M>> {
        self.view.remove_mediator::<M>()
    }

    /// Check if a [Mediator] is registered or not
    fn has_mediator<M: Mediator<Body>>(&self) -> bool {
        self.view.has_mediator::<M>()
    }
}
