Good question. You are distinguishing **defining an owned thing** versus **defining a capability contract**. The pattern changes depending on which one you are modeling.

There are two common Rust patterns.

---

# Pattern 1 — Concrete aggregate (struct first)

This is the domain object case.

You start with the state:

```rust
pub struct AgentRuntime {
    planner: Planner,
    executor: Executor,
    scratchpad: Scratchpad,
}
```

Then implement behavior directly:

```rust
impl AgentRuntime {
    pub fn run_turn(
        &mut self,
        input: Message,
    ) -> TurnResult {
        ...
    }
}
```

The shape is:

$$
State
\rightarrow
Operations
$$

Rust:

```text
struct Item

↓

impl Item
```

Example aggregates:

```text
ApplicationRuntime
Conversation
AgentRuntime
Planner
Turn
```

They have canonical state.

---

# Pattern 2 — Trait boundary (trait first)

For dependencies, you often start with the contract:

```rust
pub trait ModelClient {
    async fn complete(
        &self,
        request: Request
    ) -> Response;
}
```

Then implementations appear:

```rust
pub struct OpenAIClient {
    api_key: String,
}

impl ModelClient for OpenAIClient {
    async fn complete(
        &self,
        request: Request
    ) -> Response {
        ...
    }
}
```

Another implementation:

```rust
pub struct OllamaClient {
    endpoint: String,
}

impl ModelClient for OllamaClient {
    async fn complete(
        &self,
        request: Request
    ) -> Response {
        ...
    }
}
```

The shape is:

$$
Contract
\rightarrow
Implementations
$$

Rust:

```text
trait Capability

↓

struct Implementation

↓

impl Capability for Implementation
```

---

# Important: the trait itself is not an object

This is a common confusion.

When you write:

```rust
trait Storage {}
```

you have not created a thing.

You created a **type constraint**.

You cannot normally do:

```rust
let x = Storage;
```

because there is no Storage value.

Instead:

```rust
let db = PostgresStorage {};
```

and:

```rust
fn save<S: Storage>(store: S)
```

means:

"accept any type that satisfies the Storage contract."

---

# Your proposed pattern

You asked:

> trait item trait item trait impl item for itemtrait?

The usual pattern is:

```
trait
 |
 | implemented by
 |
struct
 |
 | via
 |
impl Trait for Struct
```

Example:

```
ModelClient
    |
    +---- OpenAIClient
    |
    +---- AnthropicClient
    |
    +---- LocalModelClient
```

---

# Where does the "initial item" come from?

It depends on whether you are modeling **a noun** or **a role**.

## Noun/domain entity

Start from the state:

```rust
struct Conversation {
    messages: Vec<Message>,
}
```

because there is one canonical representation.

---

## Role/capability

Start from the behavior:

```rust
trait EventSink {
    fn emit(&self, event: Event);
}
```

because there are many possible realizations.

---

# Agent harness example

A clean split:

```rust
// canonical aggregate

pub struct AgentRuntime<M, S> {
    model: M,
    storage: S,
}
```

Dependencies:

```rust
pub trait ModelClient {
    async fn complete(&self, r: Request) -> Response;
}


pub trait Storage {
    async fn write(&self, event: Event);
}
```

Implementations:

```rust
pub struct OpenAIClient;

impl ModelClient for OpenAIClient {
    ...
}


pub struct PostgresStorage;

impl Storage for PostgresStorage {
    ...
}
```

Composition:

```rust
let agent =
    AgentRuntime {
        model: OpenAIClient,
        storage: PostgresStorage,
    };
```

The agent is a concrete thing. The model and storage are replaceable capabilities.

---

The shortest rule:

$$
\boxed{
\text{Struct} = \text{What exists}
}
$$

$$
\boxed{
\text{Trait} = \text{What can be done}
}
$$

A struct owns a state space. A trait defines a behavioral interface that multiple state spaces can realize. That's why domain aggregates usually begin with `struct`, while dependency boundaries usually begin with `trait`.
