This is one of the deepest questions in state machine design. There are actually **two different kinds of state**, and they're often confused.

The enums I wrote are **control states** (workflow states), whereas `MissingCredentials`, `Invalid`, and `Valid` are **domain states** (knowledge or data states). They are not interchangeable.

Algebraically, they are dual in an interesting way.

---

# 1. Two different state spaces

There are really two state spaces:

$$
C
=

\text{Control State}
$$

and

$$
D
=

\text{Domain State}.
$$

Most systems combine them into one enum, but conceptually they're different.

---

## Domain state

This answers:

> **What do we currently know about the world?**

Example:

```rust
enum CredentialStatus {
    Missing,
    Cached(Token),
    Valid(UserIdentity),
    Invalid,
}
```

This represents facts.

Mathematically,

$$
CredentialStatus
\in
D.
$$

---

## Control state

This answers:

> **What is the machine currently waiting to do?**

Example:

```rust
enum AuthState {
    WaitingForLogin,
    WaitingForValidation,
}
```

This represents execution.

Mathematically,

$$
AuthState
\in
C.
$$

---

# 2. Why Temporal, BPMN, and workflow engines use control states

Imagine an OAuth login.

Initially:

```text
User clicked login.
```

Now the browser opens.

What is your credential status?

It is neither

* Valid
* Invalid
* Missing

because the process hasn't finished.

Instead the machine is

```text
WaitingForBrowserRedirect
```

That's not a fact about credentials.

It's a fact about execution.

---

# 3. Domain state cannot always describe execution

Suppose

```text
MissingCredentials
```

Now you emit

```
PromptLogin
```

Has the domain changed?

No.

The credentials are still missing.

But the machine definitely changed.

Before:

```
Idle
```

After:

```
WaitingForLogin
```

The domain stayed identical.

The control state changed.

---

# 4. This is why workflow engines look "action-oriented"

Temporal.

Cadence.

AWS Step Functions.

BPMN.

They almost always model

```text
WaitingForApproval

WaitingForPayment

WaitingForCallback

RunningActivity

Retrying

Completed
```

These are **control states**.

Not business facts.

---

# 5. The algebra

Suppose

Domain

$$
D
=

{
Missing,
Valid,
Invalid
}
$$

Control

$$
C
=

{
Idle,
WaitingLogin,
WaitingValidation
}
$$

The whole machine is

$$
C\times D.
$$

Example

$$
(
WaitingValidation,
CachedToken
)
$$

or

$$
(
WaitingLogin,
Missing
)
$$

Both dimensions evolve.

---

# 6. They evolve differently

The control machine:

$$
\delta_C:
C\times Input
\rightarrow
C
$$

The domain reducer:

$$
\delta_D:
D\times Input
\rightarrow
D
$$

Overall

$$
\delta
=

\delta_C
\times
\delta_D.
$$

This is a product automaton.

---

# 7. Why I chose workflow states

Because I was modeling the driver.

The driver needs to know

```
What should happen next?
```

not

```
Is the token valid?
```

Those are different questions.

---

# 8. There is another design

Personally, for your architecture, I think I'd actually split them.

For example

```rust
struct AuthMachine {
    control: ControlState,
    domain: CredentialState,
}
```

where

```rust
enum ControlState {
    Idle,
    WaitingForCached,
    WaitingForLogin,
    WaitingForValidation,
}
```

and

```rust
enum CredentialState {
    Missing,
    Cached(Token),
    Valid(UserIdentity),
    Invalid,
}
```

Now they're orthogonal.

---

# 9. Category-theoretically

This is actually quite elegant.

Domain state lives in one category

$$
\mathcal D.
$$

Control state lives in another

$$
\mathcal C.
$$

Your machine object is

$$
(C,D)
\in
\mathcal C
\times
\mathcal D.
$$

Transitions become

$$
(\delta_C,\delta_D).
$$

This is literally a categorical product.

---

# 10. There is an even deeper duality

You asked whether there is a duality.

I think there is.

The distinction resembles the difference between **configuration** and **observation**.

Control state answers:

> **What computation remains?**

Domain state answers:

> **What facts currently hold?**

In programming language semantics:

* **Control state** corresponds to the **continuation** or instruction pointer—where execution is in the program.
* **Domain state** corresponds to the **store** or environment—the values being manipulated.

A small-step operational semantics often models a machine configuration as

$$
(\text{Control}, \text{Store}),
$$

where one component determines the next computation and the other records the current data.

---

## For your architecture

Given how you've been decomposing systems into orthogonal lattices, I would explicitly model these as separate products rather than merging them into one enum:

```text
AuthMachine
├── ControlState      // execution progress
├── CredentialState   // business facts
└── Effects           // emitted commands
```

This yields the product state

$$
State
=

ControlState
\times
CredentialState,
$$

which is cleaner than a single mixed enum because it separates **workflow progression** from **domain truth**. That same decomposition generalizes naturally to your agent runtime: execution phases (planning, tool execution, streaming, completion) form one axis, while conversation, memory, and tool results form another. The machine then evolves by updating one or both coordinates depending on the event.
