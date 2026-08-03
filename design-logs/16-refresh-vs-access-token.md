Your design is already close to the right decomposition. The main thing to fix is that you are mixing **protocol artifacts**, **capabilities**, and **domain identities**.

The core distinction:

* `AuthorizationCode` = **proof that authorization happened**
* `AccessToken` = **capability to access resources**
* `RefreshToken` = **capability to obtain new access capabilities**

They are related, but they should absolutely be separate abstractions.

---

# 1. Is RefreshToken basically AccessToken?

No.

They are both tokens in the loose sense, but algebraically they represent different morphisms.

Access token:

$$
AccessToken
:
Client \rightarrow ResourceServer
$$

Meaning:

> "I can access this resource."

Refresh token:

$$
RefreshToken
:
Client \rightarrow AuthorizationServer
$$

Meaning:

> "I can ask for a new access token."

Different target.

---

A better model:

$$
RefreshToken
\times Client
\rightarrow
AccessToken
$$

not

$$
RefreshToken \approx AccessToken
$$

because the authorization boundary is different.

---

# 2. Should they be separate Rust types?

Yes.

This is a case where **type separation prevents security bugs**.

Bad:

```rust
struct Token {
    value: String
}
```

Then somebody can accidentally send:

```rust
api.request(refresh_token)
```

which is a security mistake.

Better:

```rust
pub struct AccessToken {
    value: String,
}

pub struct RefreshToken {
    value: String,
}
```

Now the compiler helps.

---

# 3. Create a common Token trait?

You can share behavior without sharing identity.

Example:

```rust
pub trait Token {

    fn value(&self) -> &str;

    fn expires_at(&self) -> Expiry;
}
```

Then:

```rust
impl Token for AccessToken {}

impl Token for RefreshToken {}
```

But do not make:

```rust
enum Token {
    Access(AccessToken),
    Refresh(RefreshToken)
}
```

unless you have a specific reason.

Usually these are different bounded contexts.

---

# 4. Your ScopeSet question

You are correct.

Your authorization request:

$$
Q =
ClientId
\times
RedirectUri
\times
ScopeSet
\times
State
\times
Challenge
$$

has requested scopes.

The authorization code contains **granted scopes**.

They are related but not necessarily identical.

Example:

User requests:

```text
scope=email admin payments
```

Server policy:

```text
allow=email payments
```

Therefore:

$$
GrantedScope
\subseteq
RequestedScope
$$

So I would rename.

Current:

```rust
scope: Scope
```

Better:

```rust
pub struct ScopeSet {
    values: HashSet<Scope>
}
```

Then:

```rust
pub struct AuthorizationCode {

    code: Code,

    user_id: UserId,

    client_id: ClientId,

    granted_scopes: ScopeSet,

    expiry: Expiry,
}
```

---

# 5. Do not store User and Client inside everything

This is your biggest domain modeling issue.

Currently:

```rust
AuthorizationCode{
    user: User,
    client: Client,
}
```

This creates huge aggregates.

Usually you store references.

Better:

```rust
pub struct AuthorizationCode {

    code: Code,

    user_id: UserId,

    client_id: ClientId,

    scopes: ScopeSet,

    expiry: Expiry,
}
```

Why?

Because:

$$
AuthorizationCode
\neq
User
\times
Client
$$

It references them.

The relationship is:

$$
AuthorizationCode
\rightarrow User
$$

$$
AuthorizationCode
\rightarrow Client
$$

not ownership.

---

# 6. Improved domain model

I would make:

```rust
pub struct ClientId(String);

pub struct UserId(String);

pub struct Code(String);

pub struct Jwt(String);

pub struct RedirectUri(String);

pub struct State(String);

pub struct CodeChallenge(String);
```

---

## Authorization Request

```rust
pub struct AuthorizationRequest {

    pub client_id: ClientId,

    pub redirect_uri: RedirectUri,

    pub requested_scopes: ScopeSet,

    pub state: State,

    pub code_challenge: CodeChallenge,

    pub challenge_method: ChallengeMethod,
}
```

---

## Authorization Code

```rust
pub struct AuthorizationCode {

    pub value: Code,

    pub user_id: UserId,

    pub client_id: ClientId,

    pub granted_scopes: ScopeSet,

    pub expires_at: Expiry,
}
```

---

## Token Set

OAuth usually returns a pair:

```rust
pub struct TokenSet {

    pub access_token: AccessToken,

    pub refresh_token: Option<RefreshToken>,

    pub expires_at: Expiry,
}
```

This is a useful abstraction.

Because exchange and refresh both return it.

---

# 7. IdentityProvider trait

Your trait is slightly wrong because `authorize()` usually does not return a code immediately.

The real flow is:

```
authorize request

↓

browser interaction

↓

callback

↓

code

↓

exchange
```

So split it.

---

```rust
#[async_trait]
pub trait IdentityProvider {

    type Error;


    async fn authorization_url(
        &self,
        request: AuthorizationRequest,
    )
    -> Result<AuthorizationUrl, Self::Error>;


    async fn exchange_code(
        &self,
        request: CodeExchangeRequest,
    )
    -> Result<TokenSet, Self::Error>;


    async fn refresh(
        &self,
        request: RefreshRequest,
    )
    -> Result<TokenSet, Self::Error>;


    async fn revoke(
        &self,
        request: RevokeRequest,
    )
    -> Result<(), Self::Error>;
}
```

---

# 8. Requests

## Exchange

```rust
pub struct CodeExchangeRequest {

    pub code: AuthorizationCode,

    pub verifier: CodeVerifier,

    pub client_id: ClientId,
}
```

Notice:

PKCE belongs here.

Because exchange is where it is verified.

---

## Refresh

```rust
pub struct RefreshRequest {

    pub refresh_token: RefreshToken,

    pub client_id: ClientId,

    pub requested_scopes: Option<ScopeSet>,
}
```

---

## Revoke

Revocation usually takes a token.

But the token type matters.

So:

```rust
pub enum RevocableToken {

    Access(AccessToken),

    Refresh(RefreshToken),
}
```

or separate APIs:

```rust
fn revoke_access_token()

fn revoke_refresh_token()
```

---

# 9. Authenticator layer

Your abstraction:

```rust
pub struct Authenticator<P: IdentityProvider> {
    provider: P,
}
```

is correct.

This is the orchestration layer.

It should own the workflow:

```rust
impl<P: IdentityProvider> Authenticator<P> {

    async fn login(
        &self
    ) -> Result<Session, Error> {

        let request =
            self.create_authorization_request();


        let url =
            self.provider
                .authorization_url(request)
                .await?;


        self.open_browser(url);


        let callback =
            self.receive_callback();


        let tokens =
            self.provider
                .exchange_code(callback)
                .await?;


        self.store(tokens);

        Ok(Session::authenticated())
    }
}
```

---

# 10. Final architecture

I would structure it:

```
auth/

├── domain/
│
│   ├── user.rs
│   ├── client.rs
│   ├── scope.rs
│   ├── authorization_code.rs
│   ├── token.rs
│   └── session.rs
│
├── application/
│
│   └── authenticator.rs
│
├── providers/
│
│   ├── auth0.rs
│   ├── google.rs
│   └── keycloak.rs
│
└── storage/
```

The algebraic picture:

$$
AuthorizationRequest
\overset{authorize}{\longrightarrow}
AuthorizationCode
$$

then

$$
AuthorizationCode
\times
CodeVerifier
\times
ClientId
\overset{exchange}{\longrightarrow}
TokenSet
$$

then

$$
RefreshToken
\times
ClientId
\overset{refresh}{\longrightarrow}
TokenSet
$$

and

$$
Token
\overset{revoke}{\longrightarrow}
()
$$

Your instinct to have `IdentityProvider` as a trait and `Authenticator<P>` as an orchestrator is the right architecture. The main refinement is: keep **protocol state transitions in the provider adapter**, and keep **application workflow state in Authenticator**.
