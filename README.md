# Sonic Channel

Rust client for [sonic] search backend.

We recommend you start with the [documentation].


## Installation

Add `sonic-channel = { version = "0.1" }` as a dependency in `Cargo.toml`.

`Cargo.toml` example:

```toml
[package]
name = "my-crate"
version = "0.1.0"
authors = ["Me <user@rust-lang.org>"]

[dependencies]
sonic-channel = { version = "0.1.0-rc3" }
```


## Example usage

### Search channel

```rust
use sonic_channel::*;

fn main() -> result::Result<()> {
    let mut channel = SonicChannel::connect("localhost:1491")?;

    channel.start(ChannelMode::Search, "SecretPassword")?;
    let objects = channel.query("collection", "bucket", "recipe")?;
    dbg!(objects);

    Ok(())
}
```

### Ingest channel

```rust
use sonic_channel::*;

fn main() -> result::Result<()> {
    let mut channel = SonicChannel::connect("localhost:1491")?;

    channel.start(ChannelMode::Ingest, "SecretPassword")?;
    let pushed = channel.push("collection", "bucket", "user:1", "my best recipe")?;
    dbg!(pushed);

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