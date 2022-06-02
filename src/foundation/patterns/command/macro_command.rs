use std::{cell::RefCell, fmt, rc::Rc};

use crate::{
    foundation::patterns::facade::BaseFacade,
    prelude::{Command, Facade, Interest, Notification, Notifier, Singleton},
};

/// A base [Command] implementation that executes other _Commands_.
///
/// A [MacroCommand] maintains an list of
/// [Command] Class references called _SubCommands_.
///
/// When [execute](Command::execute) is called, the [MacroCommand]
/// instantiates and calls [execute](Command::execute) on each of its _SubCommands_ turn.
/// Each _SubCommand_ will be passed a reference to the original
/// [Notification] that was passed to the [MacroCommand]'s
/// [execute](Command::execute) method.

pub struct MacroCommand<Body>
where
    Body: fmt::Debug + 'static,
{
    sub_commands: RefCell<Vec<Box<dyn Command<Body>>>>,
}

impl<Body> MacroCommand<Body>
where
    Body: fmt::Debug + 'static,
{
    /// Constructor.
    ///
    pub fn new() -> Self {
        Self {
            sub_commands: RefCell::new(Vec::new()),
        }
    }

    /// Add a `SubCommand`.
    ///
    /// The `SubCommands` will be called in First In/First Out (FIFO)
    /// order.
    ///
    /// Note that `SubCommand`s may be any [Command] implementor,
    /// [MacroCommand]'s or [SimpleCommand]'s are both acceptable.
    /// 
    /// [SimpleCommand]: super::SimpleCommand
    pub fn add_sub_command(&mut self, command: Box<dyn Command<Body>>) {
        let mut sub_commands = self.sub_commands.borrow_mut();

        sub_commands.push(command);
    }
}

impl<Body> Command<Body> for MacroCommand<Body>
where
    Body: fmt::Debug + 'static,
{
    /// Execute this [MacroCommand]'s `SubCommands`.
    ///
    /// The `SubCommands` will be called in First In/First Out (FIFO)
    /// order.
    fn execute(&self, notification: Rc<dyn Notification<Body>>) {
        let mut sub_commands = self.sub_commands.borrow_mut();
        while let Some(command) = sub_commands.pop() {
            command.execute(notification.clone());
        }
    }
}

impl<Body> Notifier<Body> for MacroCommand<Body>
where
    Body: fmt::Debug + 'static,
{
    fn send(&self, interest: Interest, body: Option<Body>) {
        log::error!("You should implement yourself MacroCommand");
        BaseFacade::<Body>::global().send(interest, body);
    }
}

impl<Body> fmt::Debug for MacroCommand<Body>
where
    Body: fmt::Debug + 'static,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MacroCommand")
            .field("sub_commands", &self.sub_commands)
            .finish()
    }
}
