use crate::authenticator::Authenticator;
use crate::domain::states::thread_manager::{ThreadManagerResult, ThreadManagerError};
use crate::domain::states::user_session::UserSessionState;



// each application is owned by a process and starts within a main thread.

pub struct ThreadManager {
    scheduler: Scheduler,
    storage: Storage, 
    telemetry: Telemetry,
    authenticator: Authenticator,
}

// todo: separate runtime states from runtime behavior
pub trait ThreadManagerTrait {
    fn run(&self) -> Result<ThreadManagerResult, ThreadManagerError>;
}

impl ThreadManagerTrait for ThreadManager {
    fn run(&self) -> Result<ThreadManagerResult, ThreadManagerError> {
        // Step 1: Authenticate the user/session

        // currently stub and id is just id but there will be claims later.
        let user_identity = self.authenticator.authenticate()
        .map_err(|_| ThreadManagerError)?;        
        // Continue with rest of startup 
        // build user session
        // load user state
        let user_session_state = UserSessionState.new(user_id);
        
        loop {

            let event =
                self.ui.next_event();

            let effects =
                self
                .conversation_controller
                .apply(
                    &mut state.conversation,
                    event,
                );

            self.execute_effects(
                state,
                effects,
            );
        }


        Ok(ThreadManagerResult {})
    }
}

