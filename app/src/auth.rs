

// is a state machine 
pub struct Authenticator{
    //stateless
}

pub trait AuthenticatorTrait{
    fn step(&self, ) -> Result<UserIdentity, AuthError>{

    }
}

pub impl Authenticator for AuthenticatorTrait{
    async fn step(&self, ) -> Result<UserIdentity, AuthError>{
        
    }
}