Your Cargo version does not support `cargo init --workspace`. My earlier command was incorrect for your installed Cargo version.

Cargo workspaces are created by manually making the **root `Cargo.toml`**. Do this instead.

From:

```bash
cd ~/p/mini-codex
```

## 1. Remove/ignore the current app package temporarily

You currently have:

```text
mini-codex/
└── app/
    └── Cargo.toml
```

Keep it for now. We will convert around it.

Create workspace root:

```bash
touch Cargo.toml
```

Edit root `Cargo.toml`:

```toml
[workspace]
members = [
    "app"
]

resolver = "2"
```

Now run:

```bash
cargo metadata
```

You should see `app` as a workspace member.

---

## 2. When you split crates

Create the crates:

```bash
mkdir crates

cargo new crates/core --lib
cargo new crates/auth --lib
cargo new crates/session --lib
cargo new crates/agent --lib
cargo new crates/runtime --lib
cargo new crates/infra --lib
```

Then update root:

```toml
[workspace]
members = [
    "app",
    "crates/core",
    "crates/auth",
    "crates/session",
    "crates/agent",
    "crates/runtime",
    "crates/infra",
]

resolver = "2"
```

---

## 3. Why your command failed

`cargo init` only creates a **package**.

The supported syntax is:

```bash
cargo init [PATH]
```

with options like:

```bash
cargo init --bin
cargo init --lib
cargo init --name myapp
```

but your Cargo release does not have:

```bash
cargo init --workspace
```

The workspace itself is just a `Cargo.toml` concept.

---

For your current project, I would actually **not move files yet**. First make the workspace work:

```text
mini-codex/
├── Cargo.toml       <-- workspace
└── app/
    ├── Cargo.toml   <-- package
    └── src/
```

Then incrementally extract:

```
app/src/domain
        |
        v
crates/core/src
```

This avoids breaking compilation while restructuring.
