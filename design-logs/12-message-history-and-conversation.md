I think you're asking two separate design questions:

1. **What is the canonical in-memory representation of a conversation?**
2. **What is the persistence model?**

Keep these separate. One is a runtime data structure; the other is a storage abstraction.

---

# 1. Message history

The first question is:

> What operations dominate?

Let's model the operations.

Let

[
H = [m_1,m_2,\ldots,m_n]
]

where the ordering is chronological.

Typical operations are:

| Operation                | Frequency    |
| ------------------------ | ------------ |
| append new message       | every turn   |
| iterate entire history   | every prompt |
| truncate oldest messages | occasionally |
| random indexing          | sometimes    |
| insert middle            | almost never |
| delete middle            | almost never |

That operation profile should determine the data structure.

---

# 2. Vec

```rust
Vec<Message>
```

Operations:

* append

[
O(1)
]

(amortized)

* indexing

[
O(1)
]

* iteration

[
O(n)
]

contiguous memory

---

This exactly matches conversation history.

Example:

```rust
pub struct ConversationState {
    messages: Vec<Message>,
}
```

Appending:

```rust
messages.push(message);
```

Very simple.

---

# 3. Why not a stack?

A stack is an **ADT**.

It supports

```text
push
pop
peek
```

A `Vec` merely implements it.

Your conversation is **not conceptually a stack** because you don't repeatedly pop messages.

The abstraction is:

[
History =
OrderedSequence
]

not

[
Stack
]

---

# 4. Why not FIFO queue?

Queue semantics:

```text
enqueue

dequeue
```

Meaning:

oldest element disappears.

That models things like:

* work queues
* schedulers
* task execution

Not conversations.

You almost never dequeue a message.

---

# 5. Ring buffer

Suppose you want:

```text
last 500 messages
```

Now:

```rust
VecDeque<Message>
```

becomes interesting.

Operations:

append back

remove front

both

[
O(1)
]

Now the runtime is maintaining a sliding window.

---

# 6. My recommendation

For your first implementation:

```rust
pub struct MessageHistory {
    messages: Vec<Message>,
}
```

Simple.

Later, if you introduce automatic context trimming:

replace internally with

```rust
VecDeque<Message>
```

The public API stays identical.

---

# 7. Don't expose Vec

Instead:

```rust
pub struct MessageHistory {
    messages: Vec<Message>,
}

impl MessageHistory {

    pub fn append(&mut self, message: Message) {
        self.messages.push(message);
    }

    pub fn iter(
        &self
    ) -> impl Iterator<Item=&Message> {
        self.messages.iter()
    }

}
```

Now you can change the implementation later without touching the rest of the code.

---

# 8. Conversation storage

Now separate runtime and persistence.

Runtime:

```text
ConversationState
```

Persistence:

```text
ConversationRepository
```

Do **not** put SQL inside Conversation.

---

# 9. Storage abstraction

I would start with:

```rust
pub trait ConversationStorage {

    fn load(
        &self,
        id: ConversationId,
    ) -> Result<ConversationState>;

    fn save(
        &self,
        conversation: &ConversationState,
    ) -> Result<()>;
}
```

Notice:

the trait describes **what**.

SQLite implements **how**.

---

# 10. SQLite implementation

Later:

```rust
pub struct SqliteConversationStorage {
    connection: Connection,
}
```

implements

```rust
impl ConversationStorage
for SqliteConversationStorage {

    ...
}
```

---

# 11. Bootstrap

Bootstrap constructs infrastructure.

```text
Bootstrap

        |

open sqlite

        |

create storage

        |

ApplicationRuntime
```

Example:

```rust
let storage =
    SqliteConversationStorage::open(
        "conversation.db"
    )?;
```

Then:

```rust
ApplicationRuntime {
    storage,
    ...
}
```

Bootstrap disappears.

---

# 12. Runtime flow

```text
authenticate

        |

UserId

        |

ConversationStorage

        |

load latest conversation

        |

ConversationState

        |

AgentRuntime
```

Notice runtime never opens SQLite.

It simply asks:

```rust
storage.load(...)
```

---

# 13. Saving

After every completed turn:

```text
Agent finishes

        |

Conversation updated

        |

storage.save(conversation)
```

The runtime orchestrates.

The storage persists.

---

# 14. Future evolution

Today:

```text
User

↓

latest conversation
```

Later:

```text
User

↓

list conversations

↓

choose conversation

↓

load conversation

↓

run
```

Nothing changes in the runtime abstraction.

Only the storage trait grows:

```rust
fn list(
    &self,
    user: UserId,
) -> Vec<ConversationMetadata>;
```

---

# 15. What I would build first

I would intentionally keep the storage interface extremely small:

```rust
trait ConversationStorage {
    fn load_latest(
        &self,
        user: UserId,
    ) -> Result<Option<ConversationState>>;

    fn save(
        &self,
        conversation: &ConversationState,
    ) -> Result<()>;
}
```

That's enough to support your current invariant:

* one active session
* one active conversation
* one agent per conversation

When you later add multiple conversations, you extend the interface (e.g., `list`, `load(id)`, `create`) without changing the runtime's core lifecycle. That keeps your abstraction stable while allowing the persistence layer to evolve independently.
