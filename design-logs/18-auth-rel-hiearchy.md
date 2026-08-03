Here is the hierarchy as a Mermaid class/domain diagram. It separates **identity**, **authorization**, **credentials**, and **OAuth protocol artifacts**.

```mermaid
classDiagram

    %% =========================
    %% Root security domain
    %% =========================

    class SecurityDomain {
    }


    SecurityDomain --> Identity
    SecurityDomain --> Authorization
    SecurityDomain --> Credential
    SecurityDomain --> OAuthProtocol


    %% =========================
    %% Identity
    %% =========================

    class Identity {
    }

    class Principal {
        +PrincipalId id
    }

    class User {
        +UserId id
        +Email email
        +UserStatus status
    }

    class ServiceAccount {
        +ServiceAccountId id
    }

    class Client {
        +ClientId id
        +String name
        +ClientType type
    }


    Identity --> Principal
    Identity --> Client

    Principal <|-- User
    Principal <|-- ServiceAccount


    %% =========================
    %% Authorization
    %% =========================

    class Authorization {
    }

    class Scope {
        +String name
    }

    class ScopeSet {
        +Set~Scope~ scopes
    }

    class AuthorizationGrant {
        +UserId user_id
        +ClientId client_id
        +ScopeSet scopes
    }


    Authorization --> Scope
    Authorization --> ScopeSet
    Authorization --> AuthorizationGrant


    AuthorizationGrant --> User
    AuthorizationGrant --> Client
    AuthorizationGrant --> ScopeSet


    %% =========================
    %% Credentials
    %% =========================

    class Credential {
        <<interface>>
        +Expiry expires_at()
    }


    class AccessToken {
        +Jwt jwt
        +PrincipalId subject
        +ClientId client_id
        +ScopeSet scopes
        +Expiry expiry
    }


    class RefreshToken {
        +Secret value
        +PrincipalId subject
        +ClientId client_id
        +ScopeSet scopes
        +Expiry expiry
        +RefreshTokenStatus status
    }


    class TokenSet {
        +AccessToken access_token
        +RefreshToken refresh_token
    }


    Credential <|-- AccessToken
    Credential <|-- RefreshToken

    TokenSet --> AccessToken
    TokenSet --> RefreshToken


    AccessToken --> Principal
    AccessToken --> Client
    AccessToken --> ScopeSet

    RefreshToken --> Principal
    RefreshToken --> Client
    RefreshToken --> ScopeSet


    %% =========================
    %% OAuth protocol artifacts
    %% =========================

    class OAuthProtocol {
    }


    class AuthorizationRequest {
        +ClientId client_id
        +RedirectUri redirect_uri
        +ScopeSet requested_scopes
        +State state
        +CodeChallenge challenge
        +ChallengeMethod method
    }


    class AuthorizationCode {
        +Code value
        +UserId user_id
        +ClientId client_id
        +ScopeSet granted_scopes
        +Expiry expiry
        +bool used
    }


    class CodeVerifier {
        +Secret value
    }


    class CodeChallenge {
        +Hash value
    }


    OAuthProtocol --> AuthorizationRequest
    OAuthProtocol --> AuthorizationCode
    OAuthProtocol --> CodeVerifier
    OAuthProtocol --> CodeChallenge


    AuthorizationRequest --> Client
    AuthorizationRequest --> ScopeSet
    AuthorizationRequest --> CodeChallenge


    AuthorizationCode --> User
    AuthorizationCode --> Client
    AuthorizationCode --> ScopeSet


    CodeVerifier --> CodeChallenge : SHA256()


    %% =========================
    %% Main flows
    %% =========================

    AuthorizationRequest --> AuthorizationCode : authorize()

    AuthorizationCode --> TokenSet : exchange()

    RefreshToken --> TokenSet : refresh()

    AccessToken --> ResourceServer : authorize()

```

The important algebraic relationships represented:

$$
AuthorizationRequest
\rightarrow
AuthorizationCode
\rightarrow
TokenSet
\rightarrow
AccessResource
$$

and:

$$
RefreshToken
\rightarrow
TokenSet
$$

while identity is referenced:

$$
AccessToken
\rightarrow
PrincipalId
$$

rather than:

$$
AccessToken = User
$$

The diagram intentionally keeps **User**, **Client**, **Scope**, and **Token** as separate objects because they belong to different domains:

* `User` answers **who**
* `Client` answers **which application**
* `ScopeSet` answers **what permissions**
* `AccessToken` answers **what capability is currently granted**
* `RefreshToken` answers **whether renewal is allowed**
* `AuthorizationCode` answers **whether the authorization ceremony completed successfully**
