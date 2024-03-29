//! Finite State Machines
//!
//! A state machine is a behavior model. It consists of a finite number of states and is therefore also 
//! called finite-state machine (FSM). Based on the current state and a given input the machine performs 
//! state transitions and produces outputs. There are basic types like Mealy and Moore machines and more 
//! complex types like Harel and UML statecharts.
//! 
//! The basic building blocks of a state machine are states and transitions. A state is a situation of a system 
//! depending on previous inputs and causes a reaction on following inputs. One state is marked as the initial state; 
//! this is where the execution of the machine starts. A state transition defines for which input a state is changed 
//! from one to another. Depending on the state machine type, states and/or transitions produce outputs.
//!
//! Consider the simple state machine above. It consists of two states, *Off* and *On*. *On* is the initial state here; 
//! it is activated when the state machine is executed. The arrows between the states denote the possible state transitions. 
//! They define for which input a state change occurs. Here, the active state is changed from *On* to *Off* for the 
//! input *buttonpressed*, and back again to *On* for the same input.
//!
//! > **Please note:** In automata theory an automaton reacts on inputs and produces outputs. There, the terms input and 
//! output are usually used for symbols which belong to an alphabet. Modern state machines use an extended definition 
//! of inputs and outputs. Inputs can be events like a button click or a time trigger while outputs are actions like 
//! an operation call or a variable assignment.
//!
//! In the following, we will extend the simple switch example to explain the differences between Mealy and Moore machines 
//! as well as Harel statecharts and UML state machines.

use std::any::TypeId;

mod fsm;
pub use fsm::*;

mod fsm_controller;
pub use fsm_controller::*;

mod integrations;
pub use integrations::*;

mod state_def;
pub use state_def::*;

///! Defines tipe_id functionality
pub trait Typed {
    ///! Retrieve TypeId
    fn type_id(&self) -> TypeId;
}

///! Defines Factory Method functionality
pub trait FactoryMethod<T> {
    ///! Create instance from Factory
    fn create(&self) -> T;
}

///! State's holder
pub type Transitions<T> = Vec<Box<dyn State<T>>>;

// todo should contain PartialEq
// pub fn contains(&self, x: &T) -> bool
// or fn any<F>(&mut self, f: F) -> bool

///! Defines State functionality for finite state machine
#[allow(unused_variables)]
pub trait State<T>: std::fmt::Debug + Typed
where
    T: FsmIntegration<T>,
{
    ///! Enter to state
    fn enter(&self, target: &T) {}

    ///! Exit from state
    fn exit(&self, target: &T) {}
}

#[cfg(test)]
mod tests {
    use std::{any::TypeId, rc::Rc};

    use super::{
        integrations::{CallbackIntegration, FsmIntegration},
        Fsm, FsmController, State, StateDef, Typed,
    };

    // pub entered: bool,
    #[derive(Default, Debug)]
    struct MockCallbackState;

    impl State<CallbackIntegration> for MockCallbackState {
        fn enter(&self, _target: &CallbackIntegration) {
            //! self.entered = true;
        }

        fn exit(&self, _target: &CallbackIntegration) {}
    }

    impl Typed for MockCallbackState {
        fn type_id(&self) -> TypeId {
            TypeId::of::<Self>()
        }
    }

    // A unit struct
    // pub entered: bool,
    #[derive(Default, Debug)]
    struct MockInjectorStateB;

    impl State<MockIntegration> for MockInjectorStateB {
        fn enter(&self, _target: &MockIntegration) {
            //! self.entered = true;
        }

        fn exit(&self, _target: &MockIntegration) {
            //! self.entered = false;
        }
    }

    impl Typed for MockInjectorStateB {
        fn type_id(&self) -> TypeId {
            TypeId::of::<Self>()
        }
    }

    // pub entered: bool,
    #[derive(Default, Debug)]
    struct MockInjectorState;

    impl State<MockIntegration> for MockInjectorState {
        fn enter(&self, _target: &MockIntegration) {
            //! self.entered = true;
        }

        fn exit(&self, _target: &MockIntegration) {
            //! self.entered = false;
        }
    }

    impl Typed for MockInjectorState {
        fn type_id(&self) -> TypeId {
            TypeId::of::<Self>()
        }
    }

    #[derive(Default, Debug, Clone)]
    struct MockIntegration;

    impl FsmIntegration<Self> for MockIntegration {
        fn transition(&self, new_state: Rc<StateDef<Self>>, old_state: Option<Rc<StateDef<Self>>>) -> bool {
            if let Some(ref old_state) = old_state {
                old_state.state.exit(self);
            }

            new_state.state.enter(self);

            true
        }
    }

    // should enter initial state
    #[test]
    fn should_enter_initial_state() {
        let fsm = Fsm::new(MockIntegration::default());

        // Seems controller should work with Rc<FSM>
        let controller = FsmController::new(fsm.clone());

        fsm.add(MockInjectorState, vec![]);

        controller.goto(MockInjectorState, None, None);
        // MockInjectorState entered should be true;
        // assert_eq!(2 + 2, 4);
    }

    // should not allow entering state if transition not added
    #[test]
    fn should_not_allow_entering_state_if_transition_not_added() {
        let fsm = Fsm::new(MockIntegration::default());

        let _controller = FsmController::new(fsm.clone());

        fsm.add(MockInjectorState, vec![]);

        // controller.goto(MockInjectorStateB)
        // Attempting to transtion to MockInjectorStateB, but state has not been added 
    }

    // should only allow adding state once
    #[test]
    fn should_only_allow_adding_state_once() {
        let fsm = Fsm::new(MockIntegration::default());

        let _controller = FsmController::new(fsm.clone());
        fsm.add(MockInjectorState, vec![]);

        // fsm.add.bind(MockInjectorState, vec![])
        // Trying to add MockInjectorState several times. Only add states once!
    }

    // should not allow entering state if transition not defined
    #[test]
    fn should_not_allow_entering_state_if_transition_not_defined() {
        let fsm = Fsm::new(MockIntegration::default());

        let controller = FsmController::new(fsm.clone());

        fsm.add(MockInjectorState, vec![]);
        fsm.add(MockInjectorStateB, vec![]);

        controller.goto(MockInjectorState, None, None);
        controller.goto(MockInjectorStateB, None, None);
        // MockInjectorStateB entered should be false;
    }

    // should enter defined transition
    #[test]
    fn should_enter_defined_transition() {
        let fsm = Fsm::new(MockIntegration::default());
        let controller = FsmController::new(fsm.clone());
        fsm.add(MockInjectorState, vec![Box::new(MockInjectorStateB::default())]);
        fsm.add(MockInjectorStateB, vec![]);

        controller.goto(MockInjectorState, None, None);
        controller.goto(MockInjectorStateB, None, None);
        // MockInjectorStateB entered should be true
        // MockInjectorState entered should be false
    }

    // should call enter on states when using callback integration
    #[test]
    fn should_call_enter_on_states_when_using_callback_integration() {
        let fsm = Fsm::new(CallbackIntegration::default());

        let controller = FsmController::new(fsm.clone());
        fsm.add(MockCallbackState, vec![]);

        controller.goto(MockCallbackState, None, None);
        // MockCallbackState entered should be true
    }
}
