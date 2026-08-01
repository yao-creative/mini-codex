This is one of the deepest design principles in Rust, and it becomes much clearer when viewed through algebra rather than "object-oriented interfaces."

The principle is:

> **Traits describe behavior required from *external dependencies*. Concrete types model *state-owning aggregates*.**

The reason is that these two things play completely different mathematical roles.

---

# Level 0 — Two different kinds of objects

Suppose we have

$$
A = \text{ApplicationRuntime}
$$

It owns state

$$
S_A.
$$

For example

$$
S_A
=

Config
\times
ToolRegistry
\times
ModelRegistry
\times
DatabasePool
\times
Telemetry.
$$

This is a product algebra.

ApplicationRuntime is literally the product

$$
A
=

\prod_i S_i.
$$

It has one canonical representation.

---

Now consider

$$
ModelClient.
$$

What is it?

It owns almost no state from the perspective of your domain.

Instead it satisfies a contract.

---

# Level 1 — State versus behavior

ApplicationRuntime

is a set

$$
A
=

{
a_1,a_2,\ldots
}
$$

whose elements are complete runtime states.

Operations are

$$
run:A\rightarrow A
$$

or

$$
create_session:
A\rightarrow
A\times Session.
$$

---

ModelClient is different.

Suppose

```rust
trait ModelClient {
    async fn complete(...);
}
```

The trait does **not** define a set of states.

Instead it defines a required operation.

---

# Level 2 — Universal algebra

Universal algebra starts with

* carrier sets
* operations

For example

$$
(\mathbb N,+,0)
$$

has

carrier

$$
\mathbb N
$$

operations

$$
+
$$

constant

$$
0.
$$

---

ApplicationRuntime is exactly this.

Carrier

$$
A
$$

Operations

```text
create_session

shutdown

run
```

These preserve the runtime invariants.

---

Traits are different.

They specify **a signature**.

A signature is

$$
\Sigma
======

(F,R)
$$

where

* (F) = function symbols
* (R) = relation symbols

For ModelClient,

$$
\Sigma_{Model}
==============

{
complete,
embed,
tokenize
}.
$$

Any type implementing the trait is a **Σ-algebra**.

For example

```text
OpenAIClient

AnthropicClient

OllamaClient
```

all interpret the same operation symbols.

They are different algebras over the same signature.

---

# Level 3 — Category theory

Suppose

$$
\mathcal M
$$

is the category of model providers.

Objects

```
OpenAI

Anthropic

Ollama

MockModel
```

Morphisms

```
configuration

wrappers

decorators
```

---

The trait

```rust
trait ModelClient
```

defines the interface that every object in

$$
\mathcal M
$$

must satisfy.

ApplicationRuntime does **not** belong in this category.

It belongs in your application category.

---

# Level 4 — Why not trait ApplicationRuntime?

Suppose you write

```rust
trait Runtime
```

How many implementations exist?

Usually

```
MyRuntime
```

Exactly one.

So the category is

```
Runtime
```

↓

```
MyRuntime
```

There is no polymorphism.

No abstraction is gained.

Mathematically

the forgetful functor

$$
U:
Runtime
\rightarrow
Set
$$

has only one object.

The abstraction has collapsed.

---

# Level 5 — Dependencies form coproducts

Suppose you support

```
OpenAI

Anthropic

Ollama
```

Then

ModelClient

is approximately

$$
OpenAI
+
Anthropic
+
Ollama.
$$

A trait abstracts over this coproduct.

---

ApplicationRuntime is not a coproduct.

It is

$$
Config
\times
ToolRegistry
\times
Database
\times
Telemetry.
$$

A product.

---

# Level 6 — Ownership

Aggregates own state.

Traits own nothing.

For example

```rust
struct Conversation {
    history,
    checkpoints,
    memories,
}
```

Conversation owns

$$
History
\times
Checkpoint
\times
Memory.
$$

A trait cannot own this.

---

# Level 7 — Dependency injection

Suppose

```rust
struct AgentRuntime {
    model: Arc<dyn ModelClient>,
}
```

Then

AgentRuntime is

$$
AgentRuntime
============

Planner
\times
Executor
\times
ModelClient.
$$

Notice

ModelClient

is simply one coordinate.

It is not the aggregate.

---

# Level 8 — Lattice interpretation

Think of refinement.

ApplicationRuntime

```text
ApplicationRuntime

├── Config

├── ToolRegistry

├── Database

└── Telemetry
```

Every child is a refinement.

---

ModelClient

instead refines horizontally

```text
             ModelClient

         /        |         \

OpenAI   Anthropic   Ollama
```

Different implementations satisfy the same specification.

One is a decomposition lattice.

The other is an implementation lattice.

---

# Level 9 — Initial and final objects

Suppose

```rust
trait Storage
```

There may exist

```
SQLite

Postgres

Memory

S3

Redis
```

These are objects in the Storage category.

ApplicationRuntime merely has a morphism

$$
ApplicationRuntime
\rightarrow
Storage.
$$

It does not become Storage.

---

# Level 10 — A useful design rule

A practical test is to ask what kind of variability you're modeling.

If you're modeling **state decomposition**, use a concrete type.

```rust
struct Conversation {
    history: History,
    checkpoints: Checkpoints,
    memories: MemoryIndex,
}
```

Algebraically, this is a product:

$$
Conversation \cong History \times Checkpoints \times MemoryIndex.
$$

There is usually one canonical representation because the aggregate owns invariants over that state.

If you're modeling **behavioral substitution**, use a trait.

```rust
trait ModelClient {
    async fn complete(&self, request: Request) -> Response;
}
```

Here the trait defines a **signature** (\Sigma), and each implementation is a (\Sigma)-algebra:

$$
\mathcal{A}*{OpenAI},;
\mathcal{A}*{Anthropic},;
\mathcal{A}_{Ollama}.
$$

From the perspective of category theory, you can view the trait as specifying a category (or more precisely, a class of algebras over a common signature), while each concrete implementation is an object in that category. `AgentRuntime` is parameterized by an object satisfying that interface, but it is **not** itself interchangeable with another runtime.

This leads to a concise heuristic:

| Question                                                           | Use a `struct` | Use a `trait` |
| ------------------------------------------------------------------ | -------------- | ------------- |
| Does it own long-lived state and invariants?                       | ✓              |               |
| Is it the canonical representation of a domain aggregate?          | ✓              |               |
| Are there multiple legitimate implementations?                     |                | ✓             |
| Is this an external capability (model, storage, tools, transport)? |                | ✓             |
| Is polymorphism required by the domain?                            |                | ✓             |

So the slogan "traits abstract dependencies, not owners" can be stated algebraically as:

* **Aggregates** are **product algebras** that own and preserve state invariants.
* **Traits** define **behavioral signatures**; implementations are **algebras over that signature**. They capture variability in *operations*, not variability in *ownership*. Keeping those roles distinct is what makes Rust APIs both idiomatic and mathematically clean.
