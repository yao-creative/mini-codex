Yes, this decomposition generally makes the system more modular because you are separating **state ownership**, **transition logic**, and **side effects**. The deeper question you are asking is actually an **aggregate boundary / ownership topology** question:

> Which states are part of one user's execution context, and which are independent resources?

Let's model the relationships first.

---

## 1. The cardinality structure

You described:

[
UserSession
\rightarrow
Many\ Conversations
]

and:

[
Conversation
\rightarrow
One\ AgentRuntime
]

So the domain graph is:

[
UserSession
\owns
{Conversation_1, Conversation_2,...,Conversation_n}
]

and:

[
Conversation_i
\owns
AgentRuntime_i
]

That is a reasonable first model.

---

## 2. Should Agent and Conversation live inside UserSession?

There are two interpretations.

### Option A: Nested ownership

```text
ApplicationState

└── UserSessionState
        |
        ├── Identity
        |
        └── Conversations
              |
              ├── Conversation A
              |       |
              |       └── AgentState
              |
              └── Conversation B
                      |
                      └── AgentState
```

This means:

[
UserSessionState =
Identity
\times
List(ConversationState)
]

Advantages:

* Easy mental model.
* User session is the root aggregate.
* Authorization is simple: session owns all user data.
* Good for a CLI where one user is active.

---

### Option B: Separate aggregates

```text
UserSession

ConversationRepository

AgentRuntimeRepository
```

The session only references:

```rust
struct UserSession {
    user_id: UserId,
}
```

Then:

```rust
Conversation {
    id: ConversationId,
    user_id: UserId,
}
```

Advantages:

* Conversations can outlive sessions.
* Easier persistence.
* Better for multiple devices.
* Less memory loaded.

---

For a CLI harness, I would start closer to **Option B conceptually but implement Option A locally**.

Meaning:

Your runtime state can hold:

```rust
struct ApplicationState {
    session: UserSessionState,
    active_conversation: Option<ConversationState>,
}
```

but persistence treats them as separate aggregates.

---

## 3. Why not put all conversations inside UserSession permanently?

Because a session is usually temporary.

Example:

Monday:

```text
Login
 |
Session #123
 |
Conversation A
```

Tuesday:

```text
Login
 |
Session #456
 |
Conversation A
```

The conversation survived the session.

Therefore:

[
ConversationLifetime

>

SessionLifetime
]

This suggests they are different aggregates.

---

## 4. Better lifetime hierarchy

Think in lifetimes:

```
Application lifetime
        |
        |
User identity lifetime
        |
        |
Session lifetime
        |
        |
Conversation lifetime
        |
        |
Turn lifetime
```

But actually:

```
User
 |
 +-- Conversation
        |
        +-- Turn
              |
              +-- Tool execution
```

is often longer-lived than:

```
User login session
```

---

## 5. AgentRuntime is tricky

I would separate:

### Agent definition

Long-lived:

```rust
Agent {
    id,
    configuration,
    tools,
    model,
}
```

### Agent execution state

Short-lived:

```rust
AgentState {
    current_plan,
    pending_tool,
    context_window,
}
```

So:

```
Conversation
    |
    +-- AgentExecutionState
```

makes sense.

But:

```
Conversation owns the Agent implementation
```

is less clean.

---

## 6. Persistence boundary

Persistence should not be owned by these aggregates.

Instead:

```text
ApplicationRuntime

        |
        |
        v

Repositories

        |
        |
        +---- UserRepository
        |
        +---- ConversationRepository
        |
        +---- EventRepository
```

Example:

```rust
trait ConversationRepository {
    async fn load(
        &self,
        id: ConversationId
    ) -> Conversation;

    async fn save(
        &self,
        conversation: &Conversation
    );
}
```

The aggregate says:

> "This is what a valid conversation is."

The repository says:

> "This is how I store it."

---

## 7. The state cross product

Your intuition is correct.

Mathematically:

[
ApplicationState =
LifecycleState
\times
UserSessionState
\times
ConversationState
\times
AgentState
]

But the important point is:

The product describes **runtime composition**, not necessarily ownership.

A Cartesian product says:

[
A\times B
]

means "these things coexist."

It does **not** imply:

[
A \owns B
]

Ownership is a separate relation.

---

## 8. I would structure your current version like this

```rust
struct ApplicationState {
    lifecycle: LifecycleState,

    session: Option<UserSessionState>,

    active_conversation: Option<ConversationState>,

    agent: Option<AgentState>,
}
```

Then:

```text
UserSession
    |
    references
    |
    user_id

Conversation
    |
    owns
    |
    turns

AgentState
    |
    belongs to
    |
    active conversation
```

Repositories:

```text
Storage
 |
 +-- UserSessionRepository
 +-- ConversationRepository
 +-- TurnRepository
 +-- MemoryRepository
```

---

So the principle:

[
\boxed{
Runtime\ composition = product
}
]

[
\boxed{
Domain\ ownership = aggregate\ boundary
}
]

[
\boxed{
Persistence = external\ adapter
}
]

Your controller-per-state-machine idea fits very well here: `UserSessionController`, `ConversationController`, and `AgentController` can each manage their own transition algebra while the application runtime composes them.
