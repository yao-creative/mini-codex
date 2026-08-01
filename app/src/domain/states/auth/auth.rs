enum AuthState {
    MissingCredentials,
    Cached(Token),
    Invalid(Token),
    Valid(UserIdentity),
}