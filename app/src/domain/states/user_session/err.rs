#[derive(Debug)]
pub enum UserSessionBuildError {
    InvalidUserId,
    SessionCreationFailed,
    // Add other error variants as necessary
}

impl std::fmt::Display for UserSessionBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserSessionBuildError::InvalidUserId => write!(f, "Invalid user ID"),
            UserSessionBuildError::SessionCreationFailed => write!(f, "Failed to create user session"),
        }
    }
}

impl std::error::Error for UserSessionBuildError {}