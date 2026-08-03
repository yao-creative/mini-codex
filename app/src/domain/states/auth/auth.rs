
enum AuthState{
    Start, //not yet authenticated
    WaitingForBrowser, // Redirect initiated
    WaitingForCallback, // User signs into redirected link and waiting for serverside authorization
    WaitingForToken, 
    Authenticated,
    Failed,
}