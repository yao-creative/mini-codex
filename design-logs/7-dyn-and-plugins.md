`dyn` in Rust means **dynamic dispatch through a trait object**.

The core distinction is:

* **Generics (`T: Trait`)** → choose implementation at compile time
* **`dyn Trait`** → choose implementation at runtime

Formally:

### Static dispatch

You have a known concrete type:

$$
f: T \rightarrow Output
$$

where `T` is fixed during compilation.

Example:

```rust
trait Storage {
    fn save(&self, data: String);
}

struct Postgres;
struct Memory;

impl Storage for Postgres {
    fn save(&self, data: String) {
        println!("postgres {}", data);
    }
}

impl Storage for Memory {
    fn save(&self, data: String) {
        println!("memory {}", data);
    }
}

fn run<S: Storage>(storage: S) {
    storage.save("hello".into());
}
```

Compiler generates:

```text
run<Postgres>()
run<Memory>()
```

This is **monomorphization**.

Advantages:

* faster
* optimized
* no runtime lookup

Cost:

* more binary size
* all types must be known

---

# What `dyn Trait` changes

A trait object says:

> "I don't know the concrete type, but I know it satisfies this interface."

Example:

```rust
fn run(storage: Box<dyn Storage>) {
    storage.save("hello".into());
}
```

Now:

```rust
let a: Box<dyn Storage> =
    Box::new(Postgres);

let b: Box<dyn Storage> =
    Box::new(Memory);

run(a);
run(b);
```

The runtime sees:

```
Box
 |
 +----------------+
 | pointer        |
 | vtable pointer |
 +----------------+

        |
        v

Postgres::save()
```

or:

```
Box
 |
 +----------------+
 | pointer        |
 | vtable pointer |
 +----------------+

        |
        v

Memory::save()
```

The vtable is basically a table of function pointers.

---

# Why do we need this?

Because sometimes the set of implementations is **open**.

Meaning:

You cannot know all possible types beforehand.

Mathematically:

Static dispatch:

$$
Implementation = {A,B,C}
$$

closed set.

Dynamic dispatch:

$$
Implementation =
{x \mid x \models Trait}
$$

open set.

---

# What are plugins?

A plugin is an external component that adds capability without modifying the core system.

Think:

$$
Core + Extension_1 + Extension_2 + ...
$$

The core defines an interface:

```rust
trait Plugin {
    fn name(&self) -> &str;

    fn execute(&self);
}
```

External developers implement it:

```rust
struct BrowserPlugin;

impl Plugin for BrowserPlugin {

    fn name(&self) -> &str {
        "browser"
    }

    fn execute(&self) {
        println!("opening browser");
    }
}
```

Your application:

```rust
struct PluginManager {
    plugins: Vec<Box<dyn Plugin>>
}

impl PluginManager {

    fn run_all(&self) {
        for plugin in &self.plugins {
            plugin.execute();
        }
    }
}
```

Now:

```rust
plugins.push(
    Box::new(BrowserPlugin)
);

plugins.push(
    Box::new(ShellPlugin)
);

plugins.push(
    Box::new(DatabasePlugin)
);
```

The manager does not know:

* BrowserPlugin exists
* ShellPlugin exists
* DatabasePlugin exists

It only knows:

$$
\forall p,\ p \in Plugin
$$

---

# Real examples of plugins

## 1. Web browser extensions

Chrome:

```
Browser Core
      |
      |
      +---- Ad blocker
      |
      +---- Password manager
      |
      +---- Developer tools
```

The browser exposes an interface.

Extensions implement it.

---

## 2. IDEs

VS Code:

```
VS Code Core

     |
     +---- Rust Analyzer
     |
     +---- Python Extension
     |
     +---- GitHub Copilot
```

The editor does not compile each extension into itself.

---

## 3. Game engines

Unity:

```
Engine

 |
 +---- Physics plugin
 |
 +---- Rendering plugin
 |
 +---- AI plugin
```

---

## 4. AI agent architecture (your case)

You could have:

```rust
trait Tool {
    fn name(&self) -> &str;

    async fn call(
        &self,
        input: ToolInput
    ) -> ToolOutput;
}
```

Then:

```rust
Vec<Box<dyn Tool>>
```

contains:

```text
[
 BrowserTool,
 PythonTool,
 SQLTool,
 SearchTool
]
```

The agent runtime does:

```rust
for tool in tools {
    tool.call(input).await;
}
```

The agent does not care whether the tool is:

* local
* remote
* HTTP
* sandboxed
* GPU accelerated

---

# When NOT to use `dyn`

Avoid:

```rust
Vec<Box<dyn Number>>
```

for a small known set:

```rust
enum Number {
    Integer(i64),
    Float(f64)
}
```

is better.

Because:

$$
enum = closed world
$$

$$
dyn Trait = open world
$$

---

# Category theory analogy

Generics:

$$
F: \mathcal{C} \rightarrow \mathcal{D}
$$

where the object type is known.

`dyn Trait`:

You are using the **forgetful functor**:

$$
U:
\text{ConcreteImplementations}
\rightarrow
\text{TraitInterface}
$$

You erase the concrete object and preserve only its morphism structure.

Concrete:

$$
PostgresMemory
$$

gets mapped to:

$$
MemoryBackend
$$

The runtime only sees the interface.

---

A practical Rust rule:

| Situation                       | Use               |
| ------------------------------- | ----------------- |
| You control all implementations | generics          |
| Performance-critical inner loop | generics          |
| A few known states              | enum              |
| External extensions             | `dyn Trait`       |
| Runtime-loaded components       | `dyn Trait`       |
| Plugin architecture             | `dyn Trait`       |
| Dependency injection boundary   | often `dyn Trait` |

For an agent harness, the typical split is:

```
Core:
    generic types

Boundaries:
    dyn Trait

Plugins:
    dyn Trait
```

because the **agent logic is closed**, but **tools/models/storage are open-ended**.
