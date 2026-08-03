


// Principal 
pub struct ClientId(String);

pub struct UserId(String);


// Credentials

pub struct Jwt(String);



// Authorization Flow
pub struct RedirectUri(String);

pub struct State(String);

pub struct CodeChallenge(String);

pub struct AuthorizationUrl(String);



#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Expiry(SystemTime);


// set of possible scopes.
pub struct ScopeSet {
    pub values: HashSet<Scope>
}


// Authorization Request
// Q=ClientId×RedirectUri×Scope×State×Challenge×ChallengeMethod

pub struct AuthorizationRequest{
    pub client_id: ClientId,
    pub redirect_uri: RedirectUri,
    pub scopes: ScopeSet,
    pub state: State,
    pub challenge: CodeChallenge,
    pub method: ChallengeMethod,
}

// Authorization Code
// K=Code×User×Client×Scope×Expiry
pub struct AuthorizationCode{
    pub code: Code,
    pub user_id: UserId,
    pub client_id: ClientId,
    pub granted_scope: ScopeSet,
    pub expiry: Expiry,
    pub used: bool,
}



// Access Token
//T=Jwt×User×Client×Scope×Expiry
// Client -> Resource Server Relation.
pub struct AccessToken{
    pub jwt: Jwt,
    pub user: UserId,
    pub scope: ScopeSet,
    pub expiry: Expiry,
}


#[derive(Debug, Clone, PartialEq)]
pub enum RefreshTokenStatus {
    Active,
    Revoked,
}
#[derive(Debug, Clone)]
pub struct Secret(String);

// TODO
pub struct RefreshToken{
    pub value: Secret,
    pub user_id: UserId,
    pub client_id: ClientId,
    pub scopes: ScopeSet,
    pub expiry: Expiry,
    pub status: RefreshTokenStatus,

} // Client -> Authorization Server Relation

pub struct RefreshRequest{
    pub refresh_token: RefreshToken,
    pub client_id: ClientId,
    pub requested_scope: ScopeSet,
}

pub struct RevokeAccessTokenRequest{
    pub token: AccessToken,
    pub client_id: ClientId,
}

pub struct RevokeRefreshTokenRequest{
    pub token: RefreshToken,
    pub client_id: ClientId,
}


pub struct CodeExchangeRequest{
    pub code: AuthorizationCode,
    pub verifier: CodeVerifier, // PKCE?
    pub client_id: ClientId,
}


pub struct TokenSet {
    pub access_token: AccessToken,
    pub refresh_token: Option<RefreshToken>,
    pub expires_at: Expiry,
}


//

