use crate::authenticator::Authenticator;
use crate::domain::states::app_runtime::{AppRuntimeResult, AppRuntimeError};



pub struct AppRuntime {
    scheduler: Scheduler,
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
        let user_identity = self.authenticator.authenticate()
        .map_err(|_| AppRuntimeError)?;        
        // Continue with rest of startup (e.g., session, log in if needed, etc.)
        // ...
        Ok(AppRuntimeResult {})
    }
}

