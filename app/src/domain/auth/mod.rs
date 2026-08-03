

// Authorization Request
// Q=ClientId×RedirectUri×Scope×State×Challenge×ChallengeMethod

struct AuthorizationRequest{
    client_id: ClientId,
    redirect_uri: RedirectUri,
    scopes: ScopeSet,
    state: State,
    challenge: CodeChallenge,
    method: ChallengeMethod,
}

// Authorization Code
// K=Code×User×Client×Scope×Expiry
struct AuthorizationCode{
    code: Code,
    user: User,
    client: Client,
    scope: Scope,
    expiry: Expiry,
}

// Access Token
//T=Jwt×User×Client×Scope×Expiry
struct AccessToken{
    jwt: Jwt,
    user: User,
    scope: Scope,
    expiry: Expiry,
}