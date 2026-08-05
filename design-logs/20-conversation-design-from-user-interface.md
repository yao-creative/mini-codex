You're asking the right question now. The key isn't "what objects should call what?" It's:

> **What is the control algebra of a conversation?**

Everything else follows from that.

---

## 1. Start from the user, not the data

Forget `MessageHistory` for a moment.

What can a **user do** to a conversation?

Initially, probably only:

$$E_C = \{Start, UserMessage, Cancel, Finish\}$$

Later:

$$+\ \{Rename, Summarize, SwitchModel, Retry, Undo, Export\}$$

Notice these are **conversation events**, not message events.

This immediately tells us that the ConversationController should operate over **ConversationEvents**, not vectors of messages.

---

## 2. What is a conversation?

Algebraically,

$$Conversation = History \times Agent \times Metadata$$

For now:

$$ConversationState = (MessageHistory, AgentState)$$

The controller is

$$\delta_C : ConversationState \times ConversationEvent \rightarrow ConversationState \times Effects$$

Notice the output.

Controllers almost never only produce state.

They also produce work.

---

## 3. Effects

Suppose:

```text
User types:

"write fibonacci"
```

ConversationController shouldn't immediately call OpenAI.

Instead:

```text
ConversationEvent::UserMessage

↓

ConversationController

↓

ConversationState'

+

Effect::RunAgent
```

The runtime executes the effect.

This keeps ConversationController pure.

---

## 4. What does ConversationController own?

I would say:

Nothing.

Literally:

```rust
pub struct ConversationController;
```

or maybe configuration.

Why?

Because it is a transition algebra.

Its input is

```rust
ConversationState
```

Its output is

```rust
ConversationState
```

It doesn't own state.

---

## 5. What should it manipulate?

Exactly these:

```rust
ConversationState
```

```rust
ConversationEvent
```

```rust
ConversationEffect
```

Not SQLite.

Not the TUI.

Not Auth.

---

## 6. What should AgentController do?

This is another transition algebra.

Input:

```text
AgentState
```

plus

```text
RunAgent
```

Output:

```text
AgentState'

+

Effects
```

Maybe:

```text
CallModel

ExecuteTool

FinishTurn
```

---

Conversation composes Agent.

Not the other way around.

---

## 7. Should ConversationController wrap AgentController?

I think yes.

Conceptually:

```text
ConversationController

        |

        +------ MessageHistory

        |

        +------ AgentController
```

Why?

Because from the user's perspective:

They interact with conversations.

Not agents.

Agent execution is an internal implementation.

---

## 8. Should it wrap MessageHistoryController?

No.

At least not yet.

History currently has one transition:

```text
append
```

That's not enough to justify another controller.

I would simply expose

```rust
history.append(...)
```

inside ConversationController.

---

## 9. The TUI

This is the most important architectural boundary.

The TUI is **not** part of Conversation.

The TUI is an adapter.

Think Hexagonal Architecture.

```text
TUI

↓

ApplicationRuntime

↓

ConversationController

↓

ConversationState
```

Never:

```text
TUI

↓

ConversationState
```

---

## 10. Runtime loop

I imagine something like

```text
TUI

↓

InputEvent

↓

ApplicationRuntime

↓

ConversationEvent

↓

ConversationController

↓

ConversationState'

+

Effects

↓

ApplicationRuntime

↓

EffectExecutor

↓

LLM

↓

ConversationEvent::AgentFinished

↓

ConversationController
```

Notice runtime is only orchestrating.

---

## 11. Why runtime instead of conversation calling the TUI?

Suppose later you add

* HTTP API

* VSCode extension

* Discord bot

* Slack

Every frontend should reuse exactly the same ConversationController.

Therefore

$$UI \notin Conversation$$

Instead:

$$UI \rightarrow ApplicationRuntime \rightarrow Conversation$$

---

## 12. Session

I agree with your earlier intuition.

Session and Conversation should be orthogonal.

Session:

```text
authenticated

current user

preferences
```

Conversation:

```text
messages

agent

metadata
```

The only relationship:

```text
Conversation.user_id

=

Session.user_id
```

Nothing more.

---

## 13. Ownership

I would have:

```text
ApplicationRuntime

owns

ConversationController
```

and

```text
ApplicationState

contains

ConversationState
```

The controller never owns the state.

---

## 14. Production systems

Most production agent frameworks end up converging to something like:

```text
Runtime

↓

Reducer

↓

State

↓

Effects

↓

Executor
```

Redux

Elm

Temporal

Akka

XState

TLA+ specifications

all have variants of

$$(State,Event) \rightarrow (State',Effects)$$

---

## 15. My only change to your current types

I would change

```rust
pub struct ConversationState {
    id: ConversationId,
    history: MessageHistoryState,
    agent: AgentState,
}
```

to

```rust
pub struct ConversationState {
    pub id: ConversationId,
    pub history: MessageHistory,
    pub agent: AgentState,
}
```

where

```rust
pub struct MessageHistory {
    messages: VecDeque<Message>,
}
```

and `MessageHistory` has methods that preserve its own local invariants.

Then define:

```rust
pub enum ConversationEvent {
    UserMessage(Message),
    AgentMessage(Message),
    Cancel,
    Finish,
}
```

and a controller of the form:

```rust
impl ConversationController {
    pub fn apply(
        &self,
        state: &mut ConversationState,
        event: ConversationEvent,
    ) -> Vec<ConversationEffect> {
        // transition logic
    }
}
```

This gives you a hierarchy of transition algebras:

$$\boxed{ApplicationRuntime \;\circ\; ConversationController \;\circ\; AgentController}$$

where:

* **ApplicationRuntime** composes state machines and routes effects.
* **ConversationController** is the aggregate transition system for a conversation.
* **AgentController** is a subordinate transition system responsible for agent execution.
* **MessageHistory** remains a rich value object until it develops an independent lifecycle that justifies its own controller.

That separation keeps your aggregate boundaries clean while leaving room to evolve toward more sophisticated agent behavior without rewriting the top-level orchestration.