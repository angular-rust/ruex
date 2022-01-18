use std::{any::TypeId, cell::RefCell, collections::HashMap, rc::Rc};

use super::{integrations::FsmIntegration, State, StateDef, Transitions, Typed};

#[derive(Default)]
pub struct FsmProps<T>
where
    T: FsmIntegration<T>,
{
    states: HashMap<TypeId, Rc<StateDef<T>>>,
    current_state: Option<Rc<StateDef<T>>>,
}

#[derive(Default, Clone)]
pub struct Fsm<T>
where
    T: FsmIntegration<T>,
{
    integration: T,
    props: Rc<RefCell<FsmProps<T>>>,
}

impl<T> Fsm<T>
where
    T: FsmIntegration<T>,
{
    // Require #![feature(const_type_id)]
    // pub const TYPE_ID: TypeId = TypeId::of::<Self>();

    pub fn new(integration: T) -> Self {
        Self {
            integration,
            props: Rc::new(RefCell::new(FsmProps {
                states: HashMap::new(),
                current_state: None,
            })),
        }
    }

    /// Triggers a state change. Transition will only happen if stateClass is in the list
    /// of transitions for the currentState.
    /// - state - The struct of the state to change to.
    pub fn goto(&self, state: impl State<T> + 'static) -> bool {
        // should be private i think
        let state_type_id = state.type_id();

        // detect transition exists
        let allowed = {
            let props = self.props.borrow();
            props
                .current_state
                .as_ref()
                .map(|current_state| {
                    current_state
                        .transitions
                        .iter()
                        .position(|item| item.type_id() == state.type_id())
                        .unwrap_or_default()
                })
                .map(|_| true)
                .unwrap_or_default()
        };

        let mut props = self.props.borrow_mut();

        let current_state = props.current_state.clone();

        match props.states.get_mut(&state_type_id) {
            Some(new_state) => {
                // State transition
                if current_state.is_some() {
                    // transition allowed
                    if allowed {
                        // make transition
                        self.integration.transition(new_state.clone(), current_state);

                        props.current_state = Some(new_state.clone());
                        log::warn!("Handle integration");

                        return true;
                    }

                    log::warn!("No transition defined from {} to {:?}", self.current_state_name(), state);
                } else {
                    // Initial state transition
                    self.integration.transition(new_state.clone(), None);
                    props.current_state = Some(new_state.clone());
                    return true;
                }
            }
            None => panic!("Attempting to transtion to {:?}, but state has not been added.", state),
        }

        false
    }

    /// Add a state with transitions to FSM.
    ///
    /// Passing the struct instead of a string reference for convinience.
    /// While setting the state will be about 4X slower, and getting about 10X, it should not be a
    /// concern unless you are going to switch state thousands of times per second.
    pub fn add(&self, state: impl State<T> + 'static, transitions: Transitions<T>) {
        let state_def = StateDef::new(state, transitions);
        let state_type_id = state_def.type_id();

        let mut props = self.props.borrow_mut();
        if props.states.contains_key(&state_type_id) {
            // TODO: Rather not have this as a runtime error, should be a macro for that.
            // panic!("Trying to add {:?} several times. Only add states once!", state_def);
            unimplemented!()
        }

        props.states.insert(state_type_id, Rc::new(state_def));
    }

    pub fn current_state_name(&self) -> String {
        // let props = self.props.borrow();
        // props
        //     .current_state
        //     .as_ref()
        //     .map(|x| format!("{:?}", x))
        //     .unwrap_or_default()
        unimplemented!()
    }
}
