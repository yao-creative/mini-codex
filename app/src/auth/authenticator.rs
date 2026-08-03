use crate::auth::identity_provider::IdentityProvider;


pub struct Authenticator<P: IdentityProvider> {
    provider: P,
}

