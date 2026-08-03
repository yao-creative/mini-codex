use crate::authenticator::Authenticator;
use crate::domain::states::app_runtime::{AppRuntimeResult, AppRuntimeError};
use crate::user_session_runtime::{UserSessionRuntime, UserSessionBuilder};





pub struct AppRuntime {
    scheduler: Scheduler,
    UserSessionRuntime:UserSessionRuntime,
    telemetry: Telemetry,
    authenticator: Authenticator,
}

// todo: separate runtime states from runtime behavior
pub trait AppRuntimeTrait {
    fn run(&self) -> Result<AppRuntimeResult, AppRuntimeError>;
}

impl AppRuntimeTrait for AppRuntime {
    fn run(&self) -> Result<AppRuntimeResult, AppRuntimeError> {
        // Step 1: Authenticate the user/session

        // currently stub and id is just id but there will be claims later.
        let user_identity = self.authenticator.authenticate()
        .map_err(|_| AppRuntimeError)?;        
        // Continue with rest of startup 
        // build user session
        // load user state
        let user_session = UserSessionBuilder.build(user_id);
        
        



        Ok(AppRuntimeResult {})
    }
}

