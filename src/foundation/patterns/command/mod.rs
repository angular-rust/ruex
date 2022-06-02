//! Commands
//!
//! The concrete Facade generally initializes the Controller with the set of
//! Notification to Command mappings needed at startup.
//! 
//! For each mapping, the Controller registers itself as an Observer for the
//! given Notification. When notified, the Controller instantiates the
//! appropriate Command. Finally, the Controller calls the Command’s
//! execute method, passing in the Notification.
//! 
//! Commands are stateless; they are created when needed and are
//! intended to go away when they have been executed. For this reason, it
//! is important not to instantiate or store references to Commands in
//! long-living objects.
//! 
//! ## Use of Macro and Simple Commands
//! 
//! Commands, like all PureMVC framework classes, implement an
//! interface, namely ICommand. PureMVC includes two ICommand
//! implementations that you may easily extend.
//! 
//! The SimpleCommand class merely has an execute method which
//! accepts an INotification instance. Insert your code in the execute
//! method and that’s it.
//! 
//! The MacroCommand class allows you to execute multiple sub-
//! commands sequentially, each being created and passed a reference
//! to the original Notification.
//! 
//! MacroCommand calls its initializeMacroCommand method from
//! within its constructor. You override this method in your subclasses
//! to call the addSubCommand method once for each Command to be
//! added. You may add any combination of SimpleCommands or
//! MacroCommands.

mod macro_command;
pub use self::macro_command::*;

mod simple_command;
pub use self::simple_command::*;
