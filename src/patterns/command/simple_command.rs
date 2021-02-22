//! A base ICommand implementation.
//!
//! Your subclass should override the execute
//! method where your business logic will handle the INotification.

#![allow(unused_variables)]

use crate::interfaces::*;
// use crate::patterns::observer::Notifier;

pub struct SimpleCommand;

impl<B> ICommand<B> for SimpleCommand {
    /// Fulfill the use-case initiated by the given INotification.
    ///
    /// In the Command Pattern, an application use-case typically
    /// begins with some user action, which results in an INotification being broadcast, which
    /// is handled by business logic in the execute method of an
    /// ICommand.
    ///
    /// * `notification` - the INotification to handle.
    fn execute(&self, notification: Box<dyn INotification<B>>) {}
}

// impl INotifier for SimpleCommand {}
