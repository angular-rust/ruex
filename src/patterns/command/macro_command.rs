//! A base ICommand implementation that executes other ICommands.
//!  
//! A MacroCommand maintains an list of
//! ICommand Class references called SubCommands.
//!
//! When execute is called, the MacroCommand
//! instantiates and calls execute on each of its SubCommands turn.
//! Each SubCommand will be passed a reference to the original
//! INotification that was passed to the MacroCommand's
//! execute method.
//!
//! Unlike SimpleCommand, your subclass
//! should not override execute, but instead, should
//! override the initializeMacroCommand method,
//! calling addSubCommand once for each SubCommand
//! to be executed.

#![allow(unused_variables)]
#![allow(dead_code)]

use crate::interfaces::*;
// use crate::patterns::observer::*;

pub struct MacroCommand {
    sub_commands: Vec<String>
}

impl MacroCommand {

    /// Constructor.
    ///
    ///
    /// You should not need to define a constructor,
    /// instead, override the initializeMacroCommand
    /// method.
    ///
    /// If your subclass does define a constructor, be
    /// sure to call super().
    pub fn new() {
        // sub_commands = new Array();
        // initializeMacroCommand();
    }

    /// Initialize the MacroCommand.
    ///
    /// In your subclass, override this method to
    /// initialize the MacroCommand's SubCommand
    /// list with ICommand class references like
    /// this:
    ///
    /// //TODO: Example here
    ///
    /// Note that SubCommands may be any ICommand implementor,
    /// MacroCommands or SimpleCommands are both acceptable.
    fn initialize_macro_command() {}

    /// Add a SubCommand.
    ///
    ///
    /// The SubCommands will be called in First In/First Out (FIFO)
    /// order.
    ///
    /// * `command_factory` - a reference to the Class of the ICommand.
    fn add_sub_command(command_factory: Box<dyn FnOnce()>) {
        // subCommands.push(command_factory);
    }
}

impl<B> ICommand<B> for MacroCommand {
    /// Execute this MacroCommand's SubCommands.
    ///
    ///
    /// The SubCommands will be called in First In/First Out (FIFO)
    /// order.
    ///
    /// * `notification` - the INotification object to be passsed to each SubCommand.
    fn execute(&self, notification: Box<dyn INotification<B>>) {
        // while ( subCommands.length > 0) {
        // 	var commandFactory : Box<dyn FnOnce()> = subCommands.shift();
        // 	var commandInstance : ICommand = new commandFactory();
        // 	commandInstance.execute( notification );
        // }
    }
}

// impl INotifier for MacroCommand {}
