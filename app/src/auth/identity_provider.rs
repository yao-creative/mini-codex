
use crate::domain::auth::{AuthorizationRequest, AuthorizationCode, RefreshRequest};
use std::result::Result;


// trait for Authenticator:
trait IdentityProvider{
    async fn authorization_url(&self, request: &AuthorizationRequest) -> Result<AuthorizationUrl, AuthorizationError> ;
    async fn exchange_code(&self, request: &CodeExchangeRequest) -> Result<TokenSet, ExchangeCodeError>;
    async fn refresh(&self, request: RefreshRequest) -> Result<TokenSet,RefreshError>;
    async fn revoke_access_token(
        &self,
        token: Token,
    ) -> Result<RevokeAccessTokenRequest, RevokeError>;
    async fn revoke_refresh_token(
        &self,
        token: Token,
    ) -> Result<RevokeRefreshTokenRequest, RevokeError>;
}

pub struct Auth0Provider;

impl IdentityProvider for Auth0Provider {
    async fn authorization_url(&self, request: &AuthorizationRequest) -> Result<AuthorizationUrl, AuthorizationError> ;
    async fn exchange_code(&self, request: &CodeExchangeRequest) -> Result<TokenSet, ExchangeCodeError>;
    async fn refresh(&self, request: RefreshRequest) -> Result<TokenSet,RefreshError>;
    async fn revoke_access_token(
        &self,
        token: Token,
    ) -> Result<RevokeAccessTokenRequest, RevokeError>;
    async fn revoke_refresh_token(
        &self,
        token: Token,
    ) -> Result<RevokeRefreshTokenRequest, RevokeError>;
}

pub struct GoogleProvider {
    client_id: ClientId,
    redirect_uri: RedirectUri,
}
