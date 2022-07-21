# Sonic Channel

[![Build](https://github.com/pleshevskiy/sonic-channel/actions/workflows/ci.yml/badge.svg)](https://github.com/pleshevskiy/sonic-channel/actions/workflows/ci.yml)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
[![Documentation](https://docs.rs/sonic-channel/badge.svg)](https://docs.rs/sonic-channel)
[![Crates.io](https://img.shields.io/crates/v/sonic-channel)](https://crates.io/crates/sonic-channel)
![Crates.io](https://img.shields.io/crates/l/sonic-channel)

Rust client for [sonic] search backend.

We recommend you start with the [documentation].

## Installation

Add `sonic-channel = { version = "1.0" }` as a dependency in `Cargo.toml`.

`Cargo.toml` example:

```toml
[package]
name = "my-crate"
version = "0.1.0"
authors = ["Me <user@rust-lang.org>"]

[dependencies]
sonic-channel = { version = "1.0", features = ["ingest"] }
```

Add `default-features = false` to dependency, if you want to exclude default
`search` channel.

## Example usage

### Search channel

Note: This example requires enabling the `search` feature, enabled by default.

```rust
use sonic_channel::*;

fn main() -> result::Result<()> {
    let channel = SearchChannel::start(
        "localhost:1491",
        "SecretPassword",
    )?;

    let objects = channel.query(QueryRequest::new(
        Dest::col_buc("collection", "bucket"),
        "recipe",
    ))?;
    dbg!(objects);

    Ok(())
}
```

### Ingest channel

Note: This example requires enabling the `ingest` feature.

```rust
use sonic_channel::*;

fn main() -> result::Result<()> {
    let channel = IngestChannel::start(
        "localhost:1491",
        "SecretPassword",
    )?;

    let dest = Dest::col_buc("collection", "bucket").obj("object:1");
    let pushed = channel.push(PushRequest::new(dest, "my best recipe"))?;
    // or
    // let pushed = channel.push(
    //     PushRequest::new(dest, "Мой лучший рецепт").lang(Lang::Rus)
    // )?;
    dbg!(pushed);

    Ok(())
}
```

### Control channel

Note: This example requires enabling the `control` feature.

```rust
use sonic_channel::*;

fn main() -> result::Result<()> {
    let channel = ControlChannel::start(
        "localhost:1491",
        "SecretPassword",
    )?;

    let result = channel.consolidate()?;
    assert_eq!(result, ());

    Ok(())
}
```

## Available features

- **default** - ["search"]
- **search** - Add sonic search mode with methods
- **ingest** - Add sonic ingest mode with methods
- **control** - Add sonic control mode with methods

[sonic]: https://github.com/valeriansaliou/sonic
[documentation]: https://docs.rs/sonic-channel
