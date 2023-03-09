use std::rc::Rc;

use super::StateDef;

// pub mod command;

// pub mod injector;

/// Defines finite state machine integration functionality
pub trait FsmIntegration<T: FsmIntegration<T>>: Clone {
    /// Makes a transition from one state to another
    fn transition(&self, new_state: Rc<StateDef<T>>, old_state: Option<Rc<StateDef<T>>>) -> bool;
}

/// Represents callback integration
#[derive(Default, Debug, Clone)]
pub struct CallbackIntegration;

impl CallbackIntegration {
    /// Create new callback integration
    pub fn new() -> Self {
        Self
    }
}

impl FsmIntegration<Self> for CallbackIntegration {
    fn transition(
        &self,
        new_state: Rc<StateDef<Self>>,
        old_state: Option<Rc<StateDef<Self>>>,
    ) -> bool {
        if let Some(ref old_state) = old_state {
            old_state.state.exit(self)
        }

        new_state.state.enter(self);

        true
    }
}
