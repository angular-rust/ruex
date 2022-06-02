use super::{integrations::FsmIntegration, Fsm, State};
// use Timer;

/// Controller to inject into application
/// 
/// Injecting FSM directly to would give access to add method, but adding states should be done during initial
/// configuration. So provide access to FsmController instead to restrict usage to only the functionality needed after
/// startup.
pub struct FsmController<T>
where
    T: FsmIntegration<T>,
{
    fsm: Fsm<T>,
}

impl<T> FsmController<T>
where
    T: FsmIntegration<T>,
{
    /// Create new FsmController
    pub fn new(fsm: Fsm<T>) -> Self {
        Self { fsm }
    }

    // wait: Option<bool> = true
    /// Goto to the state
    pub fn goto(
        &self,
        state: impl State<T> + 'static,
        guard: Option<Box<dyn Fn() -> bool>>,
        wait: Option<bool>,
    ) -> bool {
        if let Some(guard) = guard {
            let allowed = guard();
            if !allowed && !wait.unwrap_or(true) {
                return false;
            } else if !allowed {
                // FIXME: should called next tick async fashion
                self.goto(state, Some(guard), None);
                return true;
            }
        }

        self.fsm.goto(state)
    }

    /// Retrieve current state
    pub fn current_state_name(&self) -> String {
        self.fsm.current_state_name()
    }
}
