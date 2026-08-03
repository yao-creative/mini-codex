
pub struct UserId(pub String);
pub struct SessionId(pub String);

use std::time::SystemTime;


// User x Session relation. 
pub struct UserSessionState {
    user_id: UserId,
    session_id: SessionId,
    created_at: SystemTime,
}

