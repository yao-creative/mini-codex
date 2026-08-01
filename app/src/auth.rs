pub struct Authenticator{
    //stateless
}

pub trait AuthenticatorTrait{
    fn authenticate(&self, config: &Config, login: &LoginProvider) -> Result<UserIdentity, AuthError>{

    }
}

pub impl Authenticator for AuthenticatorTrait{
    async fn authenticate(&self,  config: &Config, login: &LoginProvider) -> Result<UserIdentity, AuthError>{
        
    }
}