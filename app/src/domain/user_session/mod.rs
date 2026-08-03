
pub struct UserId(pub String);
pub struct UserSessionId(String);

use std::time::SystemTime;

pub struct UserSession {
    user_id: UserId,
    user_session_id: UserSessionId,
    created_at: SystemTime,
}

