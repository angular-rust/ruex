use std::any::TypeId;

use super::{FsmIntegration, State, Transitions, Typed};

/// Represents the fsm state definition
#[derive(Debug)]
pub struct StateDef<T>
where
    T: FsmIntegration<T>,
{
    /// Represens the state
    pub state: Box<dyn State<T>>,
    /// Represens the state transitions
    pub transitions: Transitions<T>,
}

impl<T> StateDef<T>
where
    T: FsmIntegration<T>,
{
    /// Create new state definition
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
