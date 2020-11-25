# Sonic Channel

Rust client for [sonic] search backend.

We recommend you start with the [documentation].


## Installation

Add `sonic-channel = { version = "0.3" }` as a dependency in `Cargo.toml`.

`Cargo.toml` example:

```toml
[package]
name = "my-crate"
version = "0.1.0"
authors = ["Me <user@rust-lang.org>"]

[dependencies]
sonic-channel = { version = "0.3" }
```


## Example usage

### Search channel

Note: This example requires enabling the `search` feature, enabled by default.

```rust
use sonic_channel::*;

fn main() -> result::Result<()> {
    let channel = SearchChannel::start("localhost:1491", "SecretPassword")?;
    let objects = channel.query("collection", "bucket", "recipe")?;
    dbg!(objects);

    Ok(())
}
```

### Ingest channel

Note: This example requires enabling the `ingest` feature.

```rust
use sonic_channel::*;

fn main() -> result::Result<()> {
    let channel = IngestChannel::start("localhost:1491", "SecretPassword")?;
    let pushed = channel.push("collection", "bucket", "object:1", "my best recipe")?;
    // or
    // let pushed = channel.push_with_locale("collection", "bucket", "object:1", "Мой лучший рецепт", "rus")?;
    dbg!(pushed);

    Ok(())
}
```

### Control channel

Note: This example requires enabling the `control` feature.

```rust
use sonic_channel::*;

fn main() -> result::Result<()> {
    let channel = ControlChannel::start("localhost:1491", "SecretPassword")?;
    let result = channel.consolidate()?;
    assert_eq!(result, true);

    Ok(())
}
```


## Available features

* **default** - ["search"]
* **search** - Add sonic search mode with methods
* **ignite** - Add sonic ignite mode with methods
* **control** - Add sonic control mode with methods


[sonic]: https://github.com/valeriansaliou/sonic
[documentation]: https://docs.rs/sonic-channel
