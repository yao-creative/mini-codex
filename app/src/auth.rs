
use crate::domain::auth::{AuthorizationRequest, AuthorizationCode, AccessToken};


// trait for Authenticator:
trait IdentityProvider{
    async fn authorize(&self, authorization_request: &AuthorizationRequest) -> Result<AuthorizationCode, AuthorizationError> ;
    async fn exchange_code(&self, authorization_code: &AuthorizationCode) -> Result<AccessToken, ExchangeCodeError>;
    async fn refresh(&self, );
}

pub struct Auth0Provider;

impl IdentityProvider for Auth0Provider {
    async fn authorize(...);
    async fn exchange_code(...);
    async fn refresh(...);
}

