use crate::domain::states::user_session::UserSessionState;

impl UserSessionState {
    pub fn new(user_id: &str, session_id: &str, created_at: SystemTime) -> Self {
        let user_id = UserId(user_id.to_string());
        let session_id = SessionId(session_id.to_string());
        UserSessionState {
            user_id,
            session_id,
            created_at,
        }
    }
}