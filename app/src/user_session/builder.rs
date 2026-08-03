use crate::domain::session::{UserId, UserSessionId, UserSession};

use std::time::SystemTime;

pub struct UserSessionBuilder {
}


pub trait UserSessionBuilderTrait {
    fn build(&self, user_id: &UserId) -> Result<UserSession, UserSessionBuildError>;
}

impl UserSessionBuilderTrait for UserSessionBuilder {
    fn build(self, user_id: &UserId) -> UserSession {
        let user_id = UserId(user_id);
        let user_session_id = UserSessionId(uuid::Uuid::new_v4().to_string());
        let created_at = SystemTime::now();
        UserSession {
            user_id,
            user_session_id,
            created_at,
        }
    }
}