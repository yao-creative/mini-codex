use crate::domain::session::{UserId, SessionId, UserSessionState};

use std::time::SystemTime;

pub struct UserSessionStateBuilder {
}


pub trait UserSessionStateBuilderTrait {
    fn build(&self, user_id: &UserId) -> Result<UserSessionState, UserSessionStateBuildError>;
}

impl UserSessionStateBuilderTrait for UserSessionStateBuilder {
    fn build(self, user_id: &UserId) -> UserSessionState {
        let user_id = UserId(user_id);
        let session_id = SessionId(uuid::Uuid::new_v4().to_string());
        let created_at = SystemTime::now();
        UserSessionState {
            user_id,
            user_session_id,
            created_at,
        }
    }
}