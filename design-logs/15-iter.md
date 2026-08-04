I actually think the semantic explanation ("it walks through a collection") hides what an iterator *is* algebraically. Let's instead derive it from first principles.

---

# Step 1. What problem is an iterator solving?

Suppose we have a set of messages

$$
M = {m_1,m_2,\ldots,m_n}.
$$

A collection is not just a set—it has an ordering.

So a message history is better modeled as

$$
H = (m_1,m_2,\ldots,m_n)
$$

or equivalently

$$
H : {0,\ldots,n-1}\to M.
$$

Now ask:

> "How do I expose the elements one at a time?"

This is the problem an iterator solves.

---

# Step 2. The iterator is **not** the sequence

This is a very common misconception.

An iterator is **state** describing *where you currently are*.

Suppose

$$
H=(A,B,C,D).
$$

Possible iterator states are

$$
I_0
$$

(before A)

$$
I_1
$$

(before B)

$$
I_2
$$

(before C)

$$
I_3
$$

(before D)

$$
I_4
$$

(end)

Notice

$$
I
\neq
H.
$$

Instead

$$
I
=

(H,p)
$$

where

$$
p
\in
{0,\ldots,n}
$$

is the current position.

---

# Step 3. Iterator as an algebraic structure

Now define

$$
next
:
I
\rightarrow
Option(M\times I)
$$

This is **the** iterator operation.

Input:

current iterator state.

Output:

either

$$
None
$$

or

$$
Some(message,next\ iterator).
$$

Example

$$
I_0
\mapsto
(A,I_1)
$$

$$
I_1
\mapsto
(B,I_2)
$$

$$
I_2
\mapsto
(C,I_3)
$$

$$
I_3
\mapsto
(D,I_4)
$$

$$
I_4
\mapsto
None
$$

Notice something beautiful:

the iterator consumes itself into another iterator.

---

# Step 4. Why this is a coalgebra

Recall

An algebra has shape

$$
F(X)
\rightarrow
X
$$

It builds objects.

Example

$$
(Message,History)
\rightarrow
History
$$

which is exactly what

```rust
append()
```

does.

---

Iterator is the opposite.

Instead

$$
X
\rightarrow
F(X)
$$

Specifically

$$
I
\rightarrow
Option(M\times I)
$$

This **unfolds** the structure.

It reveals one observation at a time.

That is the definition of a coalgebra.

---

# Step 5. Rust exactly matches this

Rust's

```rust
Iterator
```

trait says

```rust
fn next(&mut self) -> Option<Self::Item>;
```

Semantically people read this as

> give me the next item

Algebraically it's really

```text
next :
IteratorState
→
Option(Item × IteratorState)
```

The mutable reference hides the second component.

Instead of returning

```text
(item,new_iterator)
```

Rust mutates

```text
old_iterator
```

into

```text
new_iterator.
```

So these are isomorphic:

Functional version

$$
I
\rightarrow
Option(M\times I)
$$

Rust version

$$
&mut I
\rightarrow
Option(M)
$$

because

the mutation updates

$$
I
\rightsquigarrow
I'
$$

implicitly.

---

# Step 6. Set-theoretic formulation

Iterator state

$$
I
=

H\times P
$$

where

$$
P={0,\ldots,n}
$$

Current state

$$
(H,p)
$$

Then

$$
next(H,p)
=========

\begin{cases}
None,&p=n\
(Some(H(p),(H,p+1))),&p<n
\end{cases}
$$

This is literally a function.

---

# Step 7. Universal algebra

Universal algebra studies operations.

History has operations

$$
append
:
H\times M
\rightarrow
H
$$

$$
truncate
:
H\times Budget
\rightarrow
H
$$

Iterator introduces another carrier

$$
I
$$

with operation

$$
next
:
I
\rightarrow
Option(M\times I)
$$

Notice

history and iterator are different algebras.

History supports

* append
* remove
* truncate

Iterator supports only

* next

---

# Step 8. Category theory

Suppose

History

$$
H
$$

is a finite list.

Iterator defines a coalgebra

$$
\gamma
:
I
\rightarrow
1+M\times I
$$

where

$$
1
$$

represents termination (`None`),

and

$$
+
$$

is coproduct.

This says

every iterator state is either

* finished

or

* one message together with another iterator.

This is exactly the categorical definition of a stream-like coalgebra.

---

# Step 9. Why `iter()` exists

Now we can finally describe

```rust
history.iter()
```

Algebraically.

It is **not** traversing.

It is an **initialization morphism**.

It creates the initial iterator state.

$$
iter
:
H
\rightarrow
I
$$

where

$$
iter(H)
=======

(H,0)
$$

Then repeated application of

$$
next
$$

produces

$$
(H,0)
\rightarrow
(A,(H,1))
\rightarrow
(B,(H,2))
\rightarrow
(C,(H,3))
\rightarrow
None
$$

So the complete decomposition is:

* **History** (H): the underlying data object.
* **`iter`**: (H \to I), constructs the initial iterator state.
* **Iterator state** (I = H \times P): history paired with a cursor/position.
* **`next`**: (I \to 1 + M \times I), the coalgebra that unfolds one observation and the successor state.

This decomposition is, in my view, the cleanest algebraic model of Rust iterators because it separates **construction** (`iter`) from **observation** (`next`) and makes explicit that the iterator itself is a distinct state machine rather than the collection.
