use std::{cell::RefCell, collections::HashMap, fmt, rc::Rc};

use crate::prelude::*;

/// A Singleton [Controller] implementation.
///
/// In PureMVC, the [Controller] class follows the
/// 'Command and Controller' strategy, and assumes these responsibilities:
///
/// - Remembering which [Command]'s are intended to handle which [Notification]'s.
/// - Registering itself as an [Observer] with the [View] for each [Notification]
/// that it has an [Command] mapping for.
/// - Creating a new instance of the proper [Command] to handle a given [Notification]
/// when notified by the [View].
/// - Calling the [Command]'s [execute] method, passing in the [Notification].
///
///
/// Your application must register [Command]'s with the [Controller].
/// The simplest way is to subclass [Facade],
/// and use its [register_command] method to add your registrations.
/// 
/// [execute]: Command::execute
/// [register_command]: Controller::register_command

pub struct BaseController<Body>
where
    Body: fmt::Debug + 'static,
{
    // Mapping of Notification names to Command Class references
    // Mayby use IntMap for performance
    command_map: RefCell<HashMap<Interest, Rc<dyn Command<Body>>>>,

    notify_context: Rc<BaseNotifyContext>,
}

unsafe impl<Body> std::marker::Send for BaseController<Body> where Body: fmt::Debug + 'static {}
unsafe impl<Body> std::marker::Sync for BaseController<Body> where Body: fmt::Debug + 'static {}

impl<Body> BaseController<Body>
where
    Body: fmt::Debug + 'static,
{
    /// Create instance of BaseController.
    ///
    /// Actually, you have to reimplement the [Controller] for your purposes with Singleton pattern.
    ///
    /// This [View] implementation is keept here in educational purposes only.
    ///
    pub fn new() -> Self {
        Self {
            command_map: RefCell::new(HashMap::new()),
            notify_context: Rc::new(BaseNotifyContext {}),
        }
    }

    /// Reprecent controller as [NotifyContext]
    pub fn as_context(&self) -> Rc<dyn NotifyContext> {
        self.notify_context.clone()
    }
}

impl<Body> Singleton for BaseController<Body>
where
    Body: fmt::Debug + 'static,
{
    /// [Controller] Singleton Factory method
    ///
    /// It is not possible to implement Singleton with generics.
    /// So you should implement it in your final code only.
    ///
    /// Error: use of generic parameter from outer function
    ///
    fn global() -> &'static Self {
        // static BASE_CONTROLLER_INSTANCE: OnceCell<BaseFacade<Body>> = OnceCell::new();
        // BASE_CONTROLLER_INSTANCE.get_or_init(Self::new)
        todo!("you have to reimplement the controller for your purposes")
    }
}

impl<Body> Controller<Body> for BaseController<Body>
where
    Body: fmt::Debug + 'static,
{
    fn execute_command(&self, notification: Rc<dyn Notification<Body>>) {
        let command_map = self.command_map.borrow();

        log::info!("Execute Command [BaseController] {:?}", notification);

        command_map.get(&notification.interest()).map(|command| {
            log::info!("Command [BaseController] {:?} for {:?}", command, notification);
            command.execute(notification)
        });
    }

    fn has_command(&self, interest: &Interest) -> bool {
        let command_map = self.command_map.borrow();

        command_map.contains_key(interest)
    }

    fn register_command(&self, interest: Interest, command: Rc<dyn Command<Body>>) {
        log::info!("Register Command [BaseController] {:?}", interest);
        {
            // this code should be uncommented in your final version
            // check the [Facade]::register_command
            // if !self.has_command(interest) {
            //     View::instance().register_observer(
            //         interest,
            //         Box::new(Observer::new(Box::new(|notification| self.execute_command(notification)), self.as_context())),
            //     );
            // }
        }

        self.command_map.borrow_mut().insert(interest, command);
    }

    fn remove_command(&self, interest: &Interest) {
        // if the Command is registered...
        if self.has_command(interest) {
            {
                // this code should be uncommented in your final version
                // check the [Facade]::remove_command
                // // remove the observer
                // View::instance().remove_observer(interest, &self.as_context());
            }
            self.command_map.borrow_mut().remove(interest);
        }
    }
}

impl<Body> fmt::Debug for BaseController<Body>
where
    Body: fmt::Debug + 'static,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Controller")
            //  .field("x", &self.x)
            .finish()
    }
}

#[derive(Clone, Copy)]
struct BaseNotifyContext;

impl NotifyContext for BaseNotifyContext {
    fn id(&self) -> u64 {
        0x01
    }
}

impl NotifyContext for Rc<BaseNotifyContext> {
    fn id(&self) -> u64 {
        0x01
    }
}

impl fmt::Debug for BaseNotifyContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BaseNotifyContext").field("id", &self.id()).finish()
    }
}
