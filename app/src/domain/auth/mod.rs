
// The data your callback route extracts from the query string
pub struct OAuthCallback {
    pub code: String,
    pub state: String, // CSRF token returned by Google, to verify
    pub pkce_verifier: oauth2::PkceCodeVerifier, // pulled from your stored session, not from Google
}

pub struct AuthenticatedSession {
    pub user_id: String,        // Google's `sub` claim, stable unique ID
    pub email: String,
    pub email_verified: bool,
    pub name: Option<String>,
    pub access_token: oauth2::AccessToken,
    pub refresh_token: Option<oauth2::RefreshToken>,
    pub expires_at: chrono::DateTime<chrono::Utc>, // now + expires_in
}

pub struct AuthError {
    pub kind: AuthErrorKind,
    pub message: String,
}

pub enum AuthErrorKind {
    InvalidState,     // CSRF mismatch
    ExpiredState,      // login attempt too old
    TokenExchangeFailed,
    IdTokenInvalid,    // signature/claims failed verification
    MissingIdToken,
    NetworkError,
}