You absolutely **do** specify types in traits. What you **don't** do is provide unnecessary implementation details there unless you're defining default methods.

There are three layers:

1. **Signature (the contract)** — required in the trait.
2. **Implementation** — provided in `impl`.
3. **Concrete types** — either fixed in the trait or abstracted with associated types/generics.

---

## 1. Traits specify the type-level contract

For example,

```rust
pub trait MessageHistoryController {
    fn append(
        &self,
        state: &mut MessageHistoryState,
        message: Message,
    ) -> Result<(), AppendError>;

    fn truncate(
        &self,
        state: &mut MessageHistoryState,
        budget: TokenBudget,
    ) -> Result<(), TruncateError>;
}
```

This is already a complete type specification.

Every implementation **must** satisfy exactly this signature.

The compiler proves

$$
\forall C : \texttt{MessageHistoryController},
\quad
\texttt{append}_C :
&C
\times &mut MessageHistoryState
\times Message
\to Result<(), AppendError>.
$$

No implementation can secretly accept another type.

---

## 2. Default implementations

Traits may also contain implementations.

```rust
pub trait MessageHistoryController {
    fn iter<'a>(
        &self,
        state: &'a MessageHistoryState,
    ) -> std::slice::Iter<'a, Message> {
        state.messages.iter()
    }
}
```

Then every implementation inherits it unless overridden.

---

## 3. Fixed types vs abstract types

Your current trait fixes everything:

```rust
MessageHistoryState
Message
AppendError
```

meaning every controller works on exactly those types.

Sometimes that's exactly what you want.

---

## 4. When to use associated types

Suppose later you have

```rust
ChatMessage
AudioMessage
ToolMessage
```

Instead of fixing

```rust
Message
```

you write

```rust
pub trait MessageHistoryController {
    type Message;
    type State;
    type Error;

    fn append(
        &self,
        state: &mut Self::State,
        message: Self::Message,
    ) -> Result<(), Self::Error>;
}
```

Now each implementation chooses its own types.

Example

```rust
impl MessageHistoryController for ChatHistoryController {
    type Message = ChatMessage;
    type State = ChatHistory;
    type Error = ChatError;
}
```

or

```rust
impl MessageHistoryController for AudioHistoryController {
    type Message = AudioMessage;
    type State = AudioHistory;
    type Error = AudioError;
}
```

This is much more reusable.

---

## 5. Category-theoretically

A trait is roughly an **interface specifying a family of morphisms**.

With fixed types, you're defining a single hom-set:

$$
\mathrm{Hom}(S \times M,; Result(S,E))
$$

Every implementation inhabits that same hom-set.

With associated types, the trait instead describes a **functor from implementations to types**.

Each implementation supplies

$$
(S,M,E)
$$

and therefore defines its own hom-set

$$
\mathrm{Hom}(S \times M,; Result(S,E)).
$$

So associated types parameterize the algebra itself.

---

## 6. Which should you choose?

For your harness architecture, I would **not** introduce associated types yet.

You already have domain types like:

```rust
MessageHistoryState
Message
TokenBudget
AppendError
```

These are core concepts of your system. Every implementation of `MessageHistoryController` should operate on the same message history abstraction. You're varying the **algorithm** (e.g., different truncation strategies or storage backends), not the underlying domain model.

I'd keep the trait concrete:

```rust
pub trait MessageHistoryController {
    fn append(
        &self,
        state: &mut MessageHistoryState,
        message: Message,
    ) -> Result<(), AppendError>;

    fn truncate(
        &self,
        state: &mut MessageHistoryState,
        budget: TokenBudget,
    ) -> Result<(), TruncateError>;
}
```

If you later discover you truly need controllers for fundamentally different history state types, you can refactor to associated types then. Starting concrete keeps the API simpler while still giving you full compile-time type safety.
