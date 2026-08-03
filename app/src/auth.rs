pub struct UserId(pub String);
// is a state machine 
pub struct Authenticator{
    //stateless
}

pub trait AuthenticatorTrait{
    fn authenticate(&self) -> Result<UserId, AuthError> {

    }
}
impl AuthenticatorTrait for Authenticator {
    fn authenticate(&self) -> Result<UserId, AuthError> {
        // Basic demonstration implementation
        // In a real system, replace with proper authentication logic
        Ok(UserId("demo_user".to_string()))
    }
}

