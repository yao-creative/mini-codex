use async_trait::async_trait;
use oauth2::{
    AuthUrl, TokenUrl, ClientId, ClientSecret, RedirectUrl, Scope, CsrfToken, PkceCodeChallenge,
    AuthorizationCode, RefreshToken, StandardTokenResponse, BasicTokenType,
    reqwest::async_http_client, basic::BasicClient, Url, TokenResponse,
};
use std::sync::Arc;

#[async_trait]
pub trait IdentityProvider{
    async fn login(
        &self, callback: &OAuthCallback
    ) -> result<AuthenticatedSession, AuthError>;
    async fn refresh(
        &self,
        token: oauth2::RefreshToken
    ) -> Result<AuthenticatedSession,AuthError>;
}

// Google OAuth2 IdentityProvider implementation



pub struct GoogleIdentityProvider {
    client: BasicClient,
}

impl GoogleIdentityProvider {
    pub fn new(
        client_id: String,
        client_secret: String,
        redirect_url: String,
    ) -> Self {
        let client = BasicClient::new(
            ClientId::new(client_id),
            Some(ClientSecret::new(client_secret)),
            AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string()).expect("Invalid Auth URL"),
            Some(TokenUrl::new("https://oauth2.googleapis.com/token".to_string()).expect("Invalid Token URL")),
        )
        .set_redirect_uri(RedirectUrl::new(redirect_url).expect("Invalid redirect URL"));
        Self { client }
        // create the google client:
    }

    pub fn generate_authorize_url(&self, pkce_challenge: &PkceCodeChallenge) -> (Url, CsrfToken) {
        self.client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("openid".to_string()))
            .add_scope(Scope::new("email".to_string()))
            .add_scope(Scope::new("profile".to_string()))
            .set_pkce_challenge(pkce_challenge.clone())
            .url()

    }
}


// pub enum LoginState {

//     Anonymous,

//     AwaitingAuthorization {
//         state: CsrfToken,
//         pkce_verifier: PkceCodeVerifier,
//         created_at: SystemTime,
//     },

//     AwaitingCallback {
//         state: CsrfToken,
//         pkce_verifier: PkceCodeVerifier,
//     },

//     Authenticated {
//         session_id: SessionId,
//     },

//     Expired,

//     Revoked,
// }

#[async_trait]
impl IdentityProvider for GoogleIdentityProvider {
    async fn login(
        &self,
        callback: &OAuthCallback,
    ) -> Result<AuthenticatedSession, AuthError> {
        // Anonymous -> Login() ->
        // 1. Create Auth Request
        // 2. Browser Redirect
        // 3. Callback
        // 4. Receive AuthorizationCode
        // 5. Exchange Token

        let code = AuthorizationCode::new(callback.code.clone());
        let pkce_verifier = callback.pkce_verifier.clone();

        // 
        let token_response = self
            .client
            .exchange_code(code)
            .set_pkce_verifier(pkce_verifier)
            .request_async(async_http_client)
            .await
            .map_err(|_| AuthError {})?;

        // // You would fetch the user info with the returned token here in a real application

        // Ok(AuthenticatedSession {
        //     // ... populate session with tokens, user_id, etc.
        //     // Placeholder stub:
        //     access_token: token_response.access_token().secret().to_string(),
        //     refresh_token: token_response
        //         .refresh_token()
        //         .map(|rt| rt.secret().to_string()),
        //     // other fields as required
        // })
    }

    async fn refresh(
        &self,
        token: oauth2::RefreshToken,
    ) -> Result<AuthenticatedSession, AuthError> {
        // let token_response = self
        //     .client
        //     .exchange_refresh_token(&token)
        //     .request_async(async_http_client)
        //     .await
        //     .map_err(|_| AuthError {})?;

        // // With the new tokens, update session info as needed.

        // Ok(AuthenticatedSession {
        //     access_token: token_response.access_token().secret().to_string(),
        //     refresh_token: token_response
        //         .refresh_token()
        //         .map(|rt| rt.secret().to_string()),
        //     // other fields as required
        // })
    }
}

// -- Structures assumed to exist elsewhere --
// These stubs are for compilation/reference only and should be replaced with real ones.

pub struct OAuthCallback {
    pub code: String,
    pub pkce_verifier: oauth2::PkceCodeVerifier,
}

pub struct AuthenticatedSession {
    pub access_token: String,
    pub refresh_token: Option<String>,
    // additional fields as needed (user_id, provider, issued_at, etc.)
}

pub struct AuthError {} // Replace with your detailed error structure.