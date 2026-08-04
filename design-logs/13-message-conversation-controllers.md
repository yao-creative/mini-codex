You are touching a real design tension:

> Do I model subcomponents as independent state machines with controllers, or do I keep them as data structures inside a larger aggregate and let the aggregate enforce invariants?

This is essentially the **aggregate boundary vs compositional state machine** problem.

For `MessageHistory`, I would be careful not to make it a full controller yet. There are several levels of abstraction.

---

# Option 1 — Value Object (recommended first)

Treat `MessageHistory` as a domain value with enforced invariants.

Structure:

```text
Conversation
    |
    owns
    |
    MessageHistory
```

Rust:

```rust
pub struct ConversationState {
    id: ConversationId,
    history: MessageHistory,
}


pub struct MessageHistory {
    messages: Vec<Message>,
}
```

The history controls its own invariants:

```rust
impl MessageHistory {

    pub fn append(
        &mut self,
        message: Message,
    ) {
        self.messages.push(message);
    }

    pub fn iter(
        &self,
    ) -> impl Iterator<Item=&Message> {
        self.messages.iter()
    }
}
```

The invariant:

[
MessageHistory =
\text{ordered valid sequence of Messages}
]

is local.

Conversation does not know how the vector works.

---

# Option 2 — Conversation aggregate controller

Here:

```text
ConversationController

        |
        v

ConversationState

        |
        owns

MessageHistory
```

Example:

```rust
pub struct ConversationController;


impl ConversationController {

    pub fn append_message(
        conversation: &mut ConversationState,
        message: Message,
    ) {
        conversation
            .history
            .append(message);
    }
}
```

Now:

[
ConversationState'
==================

f(ConversationState, Event)
]

The conversation is the state machine.

Example:

```rust
enum ConversationEvent {
    UserMessage(Message),
    AssistantMessage(Message),
}
```

Transition:

```rust
fn apply(
    state: &mut ConversationState,
    event: ConversationEvent
)
```

This is closer to event sourcing / TLA+ style.

---

# Option 3 — MessageHistory as its own state machine

You are proposing:

```text
ConversationController

        |
        v

MessageHistoryController

        |
        v

MessageHistoryState
```

Mathematically:

[
ConversationState
=================

MessageHistoryState
\times
OtherState
]

Then:

[
MessageHistoryController:
(H,e)\rightarrow H'
]

Example:

```rust
pub struct MessageHistoryController;

impl MessageHistoryController {

    pub fn apply(
        &self,
        state: &mut MessageHistoryState,
        event: MessageEvent,
    ) {
        ...
    }
}
```

This is valid.

But the question is:

Does MessageHistory have independent transitions?

For example:

```
append message
remove message
summarize
compact
truncate
```

If yes, it may deserve this.

If it is only:

```
push message
iterate messages
```

then a controller is probably too much ceremony.

---

# Algebraically

A controller represents a transition algebra:

[
F:S\times E\rightarrow S
]

where:

* (S) = state space
* (E) = events

A value object is closer to an algebraic data type:

[
MessageHistory =
List(Message)
]

with operations:

[
append:
History\times Message \rightarrow History
]

The difference:

Controller:

[
\text{external transition system}
]

Value object:

[
\text{internal algebra}
]

---

# Builder vs Controller

You mentioned "builder than controller most things".

They solve different problems.

## Builder

Construction:

[
\emptyset \rightarrow ValidState
]

Example:

```rust
ConversationBuilder::new()
    .user(user_id)
    .build()
```

It answers:

> How do I create a valid object?

---

## Controller

Evolution:

[
State \times Event \rightarrow State'
]

Example:

```rust
conversation_controller
    .handle(event)
```

It answers:

> How does a valid object change?

---

You need both.

---

# For your current system I would do:

```text
Conversation
|
├── ConversationState
|
└── MessageHistory
        |
        └── Vec<Message>
```

Then:

```text
ConversationBuilder

creates

ConversationState


ConversationController

changes

ConversationState
```

---

Code:

```rust
pub struct ConversationState {
    id: ConversationId,
    history: MessageHistory,
}


pub struct ConversationController;


impl ConversationController {

    pub fn receive_message(
        &self,
        conversation: &mut ConversationState,
        message: Message,
    ) {
        conversation
            .history
            .append(message);
    }
}
```

---

# Later evolution path

If history becomes complex:

Today:

```text
MessageHistory
    |
    Vec<Message>
```

Tomorrow:

```text
MessageHistory
    |
    ├── append
    ├── summarize
    ├── compact
    ├── checkpoint
    └── restore
```

Then extract:

```text
MessageHistoryController
```

without changing:

```rust
ConversationState {
    history: MessageHistory
}
```

---

# Other alternatives

## 1. Event sourcing

Instead of storing:

```rust
Vec<Message>
```

store:

```rust
Vec<ConversationEvent>
```

Then:

[
State = reduce(events)
]

Example:

```
UserSent("hello")
AssistantResponded("hi")
ToolCalled(...)
```

Pros:

* auditability
* replay
* debugging

Cons:

* more complexity

---

## 2. CQRS

Separate:

Write model:

```text
ConversationAggregate
```

Read model:

```text
ConversationView
```

Useful when:

* millions of conversations
* search
* analytics

Too early now.

---

## 3. Reducer style

Very Rust-friendly:

```rust
fn reduce(
    state: ConversationState,
    event: ConversationEvent
) -> ConversationState
```

This gives you explicit inductive reasoning:

Base:

[
Valid(initial)
]

Step:

[
Valid(s)\land Event
\Rightarrow Valid(reduce(s,e))
]

This maps nicely to your TLA+/state machine interests.

---

My recommendation for your current agent harness:

```
ApplicationRuntime
    |
    |
ConversationController
    |
    |
ConversationState
    |
    |
MessageHistory (value object)
```

Do **not** make every nested structure a controller. Make something a controller when it has:

1. its own lifecycle,
2. meaningful transitions,
3. invariants that are easier to prove separately.

`Conversation` qualifies.

`MessageHistory` probably starts as a value object and can evolve into its own state machine later.
