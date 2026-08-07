Your intent is **algebraic decomposition of an initialization morphism**: you want to see `Bootstrap::build` not as "some Rust code", but as a composition of typed transformations from **external input space** into the final runtime object space.

The function:

```rust
impl Bootstrap {
    pub fn build(args: CliArgs) -> anyhow::Result<Host> {
        ConfigBuilder::build(args)
            .and_then(|config| {
                Storage::new(&config.database).map(|storage| (config, storage))
            })
            .map(|(config, storage)| {
                let auth = Authenticator::new(config.auth);
                let app = ApplicationRuntime::new(storage, auth);

                Host {
                    app,
                    tui: TuiRuntime::new(),
                    engine: Engine::new(),
                }
            })
    }
}
```

is mathematically:

$$
\mathrm{Bootstrap}: CliArgs \rightarrow Result<Host>
$$

but internally it factors into smaller morphisms.

---

## 1. Domain decomposition (Set theory)

Define sets:

$$
A = CliArgs
$$

CLI input space.

$$
C = Config
$$

Validated configuration space.

$$
D = DatabaseConfig
$$

Database subset of configuration.

$$
S = Storage
$$

Concrete storage implementations.

$$
Au = Authenticator
$$

Authentication subsystem.

$$
AR = ApplicationRuntime
$$

Application control state.

$$
H = Host
$$

Complete executable system.

---

The construction is a chain:

$$
A
\xrightarrow{config}
C
\xrightarrow{storage}
C \times S
\xrightarrow{runtime}
AR
\xrightarrow{host}
H
$$

---

## 2. `ConfigBuilder::build`

Rust:

```rust
ConfigBuilder::build(args)
```

signature:

```rust
CliArgs -> Result<Config>
```

Mathematically:

$$
f : A \rightarrow Result(C)
$$

It is a **partial function** because invalid CLI arguments do not produce a configuration.

Example:

$$
f(a)=
\begin{cases}
c & \text{valid args}\
Error & \text{invalid args}
\end{cases}
$$

The `Result` type is:

$$
Result(C) \cong C + Error
$$

where `+` is a coproduct (sum type).

So:

$$
f:A\rightarrow C+E
$$

---

## 3. `and_then`

This:

```rust
.and_then(|config| {...})
```

is Kleisli composition.

Normally composition:

$$
g \circ f
$$

requires:

$$
f:A\rightarrow C
$$

and

$$
g:C\rightarrow D
$$

But here:

$$
f:A\rightarrow C+E
$$

so normal composition does not type check.

`and_then` lifts composition into the `Result` category:

$$
g:C\rightarrow D+E
$$

becomes:

$$
g^*:C+E\rightarrow D+E
$$

Therefore:

$$
g^* \circ f
$$

is valid.

In Rust:

```rust
ConfigBuilder::build(args)
    .and_then(storage_creation)
```

means:

$$
A
\rightarrow
C+E
\rightarrow
(C\times S)+E
$$

---

## 4. Storage construction

This:

```rust
Storage::new(&config.database)
```

is:

$$
storage:D\rightarrow S+E
$$

but because you still need the config:

```rust
map(|storage| (config, storage))
```

creates a product:

$$
C\times S
$$

So:

$$
C
\rightarrow
C\times S
$$

The full morphism:

$$
A
\rightarrow
C+E
\rightarrow
(C\times S)+E
$$

---

## 5. Final `.map`

This:

```rust
.map(|(config, storage)| {...})
```

is not Kleisli composition.

It is functor mapping.

`Result` is a functor:

$$
Result : Set \rightarrow Set
$$

Given:

$$
h:C\times S\rightarrow H
$$

Functor lifting gives:

$$
Result(h):
Result(C\times S)
\rightarrow
Result(H)
$$

Rust:

```rust
.map(|x| build_host(x))
```

is:

$$
Result(f)
$$

---

## 6. Expand the final builder

The closure:

```rust
|(config, storage)| {
    let auth = Authenticator::new(config.auth);
    let app = ApplicationRuntime::new(storage, auth);

    Host {
        app,
        tui: TuiRuntime::new(),
        engine: Engine::new(),
    }
}
```

is actually several morphisms:

### Auth construction:

$$
AuthConfig\rightarrow Authenticator
$$

### Application construction:

$$
Storage \times Authenticator
\rightarrow ApplicationRuntime
$$

### Host construction:

$$
ApplicationRuntime
\times TuiRuntime
\times Engine
\rightarrow Host
$$

So the complete algebra:

$$
(C\times S)
\rightarrow
(C\times S)
\times Auth
\rightarrow
ApplicationRuntime
\rightarrow
Host
$$

---

## 7. Category diagram

The entire bootstrap:

$$
\begin{array}{ccc}
CliArgs
&\xrightarrow{ConfigBuilder}&
Config
\
&&
\downarrow Storage::new
\
&&
Config\times Storage
\
&&
\downarrow RuntimeConstruction
\
&&
ApplicationRuntime
\
&&
\downarrow HostConstruction
\
&&
Host
\end{array}
$$

---

## 8. Why this architecture is mathematically clean

`Bootstrap` is not a controller.

It is a **catamorphism over initialization data**.

It consumes a finite description:

$$
CliArgs
$$

and folds it into a runtime algebra:

$$
Host
$$

You can think of:

```
CliArgs
    |
    v
 Config
    |
    v
 Resources
    |
    v
 Runtime
    |
    v
 Host
```

as an initial algebra:

$$
Build : Input \rightarrow System
$$

The important invariant is:

[
\boxed{
\text{Bootstrap owns construction order, not runtime behavior}
}
]

After construction:

```
Bootstrap
    |
    creates
    |
    Host
    |
    runs
    |
    ApplicationRuntime
```

The `Host` owns the operational morphisms (loops, events, scheduling). The `Bootstrap` only proves that:

$$
Host \in ValidSystem
$$

before execution begins.
