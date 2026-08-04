Your intuition is close, but the exact mechanism is slightly different.

`BufReader::lines()` **does not mean "give me a buffer-sized line"**.

It means:

> "Keep reading from the buffered reader until I encounter a newline delimiter (`\n`) or EOF, then return the complete line."

The buffer size only controls **how much data is fetched from the OS at once**, not the size of the returned item.

---

## The layers

Think of it as:

$$
Disk \rightarrow Kernel Buffer \rightarrow BufReader Buffer \rightarrow Lines Iterator \rightarrow String
$$

Example file:

```
hello world\n
this is a second line\n
third line
```

---

### BufReader

Suppose:

```rust
let reader = BufReader::with_capacity(8, file);
```

Buffer size = 8 bytes.

It reads:

```
[hello wo]
```

into memory.

Not because the line is 8 bytes.

Because the OS read syscall gets 8 bytes.

---

Now:

```rust
reader.lines().next()
```

asks:

"Give me one line."

The iterator checks:

```
buffer:
hello wo
```

No `\n` yet.

So it reads more:

```
buffer:
hello world\n
```

Now it finds newline:

```
return:

"hello world"
```

---

So the returned size is:

$$
LineSize = distance(current\ position,\ newline)
$$

not:

$$
min(BufferSize, LineSize)
$$

---

## What if one line is huge?

Example:

```
AAAAAAAAAAAAAAAAAAAA....(10GB)...\n
```

with:

```rust
BufReader::with_capacity(8, file)
```

The behavior:

```
read 8 bytes
append

read 8 bytes
append

read 8 bytes
append

...

find \n
return entire String
```

So:

```
Buffer:
8KB

Returned line:
10GB
```

The iterator may allocate a huge `String`.

This is an important production consideration.

---

# Algebraically

`BufReader`:

$$
Bytes \rightarrow Buffered(Bytes)
$$

`lines()`:

$$
Buffered(Bytes)
\rightarrow
Iterator(String)
$$

The iterator state contains:

$$
S=
(
buffer,
cursor,
reader
)
$$

Then:

$$
next(S)
=======

(line,S')
$$

where:

* scan until delimiter
* update cursor
* return element

---

# Compare with `chunks()`

A chunk iterator behaves differently:

```rust
reader.bytes()
```

or:

```rust
stream.chunk(4096)
```

gives:

```
[4096 bytes]
[4096 bytes]
[4096 bytes]
```

Here:

$$
OutputSize = BufferSize
$$

But:

```rust
lines()
```

gives:

```
[line1]
[line2]
[line3]
```

where:

$$
OutputSize = DelimiterBoundary
$$

---

# Production implication

For logs:

```text
2026-08-04 INFO user logged in
2026-08-04 ERROR something failed
```

`lines()` is good.

For:

* video
* images
* embeddings
* model tensors
* network packets

you usually want:

```rust
chunks()
```

because the natural boundary is size.

---

# In your harness example

For message history:

You probably do **not** want:

```rust
for message in reader.lines()
```

because conversation messages have semantic boundaries.

You want:

```
Storage
 |
 |
MessageIterator
 |
 |
Message
```

where:

```rust
struct MessageIterator {
    cursor: MessageId,
    storage: Database
}
```

Then:

```rust
impl Iterator for MessageIterator {
    type Item = Message;

    fn next(&mut self)
        -> Option<Message>
    {
        self.storage.fetch_after(self.cursor)
    }
}
```

Your delimiter is not:

```
\n
```

Your delimiter is:

```
message_id boundary
```

The same iterator algebra applies:

$$
State \rightarrow Message + State
$$

but the state transition is database-driven instead of byte-driven.

---

So the key distinction:

| Abstraction                 | Boundary               |
| --------------------------- | ---------------------- |
| `BufReader`                 | byte buffer            |
| `chunks()`                  | fixed size             |
| `lines()`                   | newline delimiter      |
| database cursor             | row boundary           |
| Kafka consumer              | message offset         |
| your MessageHistoryIterator | message ID / timestamp |

The iterator abstraction hides **what advances the state**. The producer can be a file, database, network socket, Kafka partition, or your agent memory store.
