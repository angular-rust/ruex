use std::rc::Rc;

use super::StateDef;

// pub mod command;

// pub mod injector;

pub trait FsmIntegration<T: FsmIntegration<T>>: Clone {
    fn transition(&self, new_state: Rc<StateDef<T>>, old_state: Option<Rc<StateDef<T>>>) -> bool;
}

#[derive(Default, Debug, Clone)]
pub struct CallbackIntegration;

impl CallbackIntegration {
    pub fn new() {}
}

impl FsmIntegration<Self> for CallbackIntegration {
    fn transition(&self, new_state: Rc<StateDef<Self>>, old_state: Option<Rc<StateDef<Self>>>) -> bool {
        if let Some(ref old_state) = old_state {
            old_state.state.exit(self)
        }

        new_state.state.enter(self);

        true
    }
}
