use std::any::TypeId;

use super::{FsmIntegration, State, Transitions, Typed};

#[derive(Debug)]
pub struct StateDef<T>
where
    T: FsmIntegration<T>,
{
    pub state: Box<dyn State<T>>,
    pub transitions: Transitions<T>,
}

impl<T> StateDef<T>
where
    T: FsmIntegration<T>,
{
    pub fn new(state: impl State<T> + 'static, transitions: Transitions<T>) -> Self {
        Self {
            state: Box::new(state),
            transitions,
        }
    }
}

impl<T> Typed for StateDef<T>
where
    T: FsmIntegration<T>,
{
    fn type_id(&self) -> TypeId {
        self.state.type_id()
    }
}
