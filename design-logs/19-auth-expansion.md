Your current model is mixing **identity**, **authorization**, and **capability artifacts**. The clean algebraic hierarchy is to separate:

1. **Who** (identity domain)
2. **What is allowed** (authorization domain)
3. **Proof/capability artifacts** (credential domain)
4. **Protocol artifacts** (OAuth flow domain)

A useful decomposition:

$$
SecurityDomain
=

Identity
\times
Authorization
\times
Credential
\times
Protocol
$$

---

# 1. Identity domain

Identity answers:

> "Who is this?"

The hierarchy:

$$
Identity
\supset
Principal
\supset
User
$$

A principal is the general authenticated entity.

It can be:

* human user
* service account
* machine
* workload

Rust:

```rust
pub trait Principal {
    fn id(&self) -> PrincipalId;
}
```

Concrete:

```rust
pub struct User {

    pub id: UserId,

    pub profile: UserProfile,

}
```

```rust
pub struct ServiceAccount {

    pub id: ServiceAccountId,

}
```

---

## User identity

```rust
pub struct UserId(String);


pub struct User {

    pub id: UserId,

    pub email: Email,

    pub status: UserStatus,

}
```

The important point:

A `User` is not an authorization token.

It is an identity.

---

# 2. Client identity domain

OAuth has another actor:

the application requesting access.

So:

$$
Principal
=

User
+
Client
$$

or:

$$
Identity
=

HumanIdentity
+
ApplicationIdentity
$$

Rust:

```rust
pub struct ClientId(String);


pub struct Client {

    pub id: ClientId,

    pub name: String,

    pub client_type: ClientType,

}
```

---

# 3. Scope domain

Scope answers:

> "What actions are allowed?"

A scope is an atomic permission.

$$
Scope
=

Permission
$$

Example:

```text
profile.read

email.read

files.write
```

Rust:

```rust
pub struct Scope(String);
```

---

A set of scopes:

$$
ScopeSet = \mathcal{P}(Scope)
$$

Power set of scopes.

Rust:

```rust
pub struct ScopeSet {

    scopes: HashSet<Scope>

}
```

---

Example:

```rust
ScopeSet {
    profile.read,
    email.read,
}
```

---

# 4. Authorization domain

Now combine identity and permissions.

Authorization relationship:

$$
(User, ScopeSet)
\rightarrow
Permission
$$

A grant:

```rust
pub struct AuthorizationGrant {

    pub user_id: UserId,

    pub client_id: ClientId,

    pub scopes: ScopeSet,

}
```

Meaning:

"This client may act with these permissions on behalf of this user."

---

# 5. Credential domain

Now tokens.

A token is not identity.

It is evidence/capability.

Hierarchy:

$$
Credential
\supset
Token
$$

```rust
pub trait Credential {

    fn expires_at(&self) -> Expiry;

}
```

---

## Access Token

Purpose:

$$
AccessToken
\rightarrow
ResourceServer
$$

Your current:

```rust
pub struct AccessToken {

    jwt: Jwt,

    user: User,

    scope: Scope,

    expiry: Expiry,

}
```

I would change it.

Do not embed `User`.

Use references:

```rust
pub struct AccessToken {

    pub jwt: Jwt,

    pub subject: PrincipalId,

    pub client_id: ClientId,

    pub scopes: ScopeSet,

    pub expiry: Expiry,

}
```

Because the token carries claims about identity.

It is not the identity itself.

---

Algebraically:

$$
AccessToken
=

Jwt
\times
PrincipalId
\times
ClientId
\times
ScopeSet
\times
Expiry
$$

---

# 6. Refresh Token

Purpose:

$$
RefreshToken
\rightarrow
AccessToken
$$

It is a different capability.

```rust
pub struct RefreshToken {

    pub value: Secret,

    pub subject: PrincipalId,

    pub client_id: ClientId,

    pub scopes: ScopeSet,

    pub expiry: Expiry,

    pub status: RefreshTokenStatus,

}
```

Algebraically:

$$
RefreshToken
=

Secret
\times
PrincipalId
\times
ClientId
\times
ScopeSet
\times
Expiry
\times
Status
$$

---

# 7. TokenSet

OAuth exchange returns:

$$
AuthorizationCode
\rightarrow
TokenSet
$$

```rust
pub struct TokenSet {

    pub access_token: AccessToken,

    pub refresh_token: Option<RefreshToken>,

}
```

---

# 8. Authorization Code

This is not a credential.

It is a protocol continuation artifact.

Purpose:

$$
AuthorizationCode
\rightarrow
TokenSet
$$

```rust
pub struct AuthorizationCode {

    pub value: Code,

    pub user_id: UserId,

    pub client_id: ClientId,

    pub scopes: ScopeSet,

    pub expiry: Expiry,

    pub used: bool,

}
```

Algebraically:

$$
AuthorizationCode
=

Code
\times
UserId
\times
ClientId
\times
ScopeSet
\times
Expiry
\times
Used
$$

---

# 9. PKCE domain

PKCE binds the authorization request to the token exchange.

Hierarchy:

$$
Verifier
\rightarrow
Challenge
$$

```rust
pub struct CodeVerifier(
    Secret
);


pub struct CodeChallenge(
    Hash
);
```

Relationship:

$$
SHA256(CodeVerifier)=CodeChallenge
$$

---

# 10. Complete hierarchy

Putting everything together:

$$
\boxed{
Security
}
$$

splits:

$$
Security
=

Identity
\times
Authorization
\times
Credential
\times
Protocol
$$

## Identity

$$
Identity
=

Principal
\times
Client
$$

where:

$$
Principal
=

User
+
ServiceAccount
$$

---

## Authorization

$$
Authorization
=

Scope
\times
ScopeSet
\times
Grant
$$

---

## Credentials

$$
Credential
=

AccessToken
+
RefreshToken
$$

---

## OAuth Protocol artifacts

$$
Protocol
=

AuthorizationRequest
+
AuthorizationCode
+
PKCE
$$

---

# Rust module structure

I would make the domain look like:

```text
auth/

domain/

    identity/
        user.rs
        client.rs
        principal.rs

    authorization/
        scope.rs
        scope_set.rs
        grant.rs

    credential/
        access_token.rs
        refresh_token.rs
        token_set.rs

    oauth/
        authorization_request.rs
        authorization_code.rs
        pkce.rs
```

---

The key correction to your current model:

```rust
pub struct AccessToken{
    jwt: Jwt,
    user: User,
    scope: Scope,
    expiry: Expiry,
}
```

should become closer to:

```rust
pub struct AccessToken {

    jwt: Jwt,

    subject: PrincipalId,

    client_id: ClientId,

    scopes: ScopeSet,

    expiry: Expiry,

}
```

because the algebraic relationship is:

$$
Token
\rightarrow
Claims
\rightarrow
IdentityReference
$$

not:

$$
Token
=

Identity
$$

The token **asserts** identity and permissions; it does not contain or own them.
