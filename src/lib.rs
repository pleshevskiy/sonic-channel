//! # Sonic Channel
//! Rust client for [sonic] search backend.
//! 
//! 
//! ## Example usage
//! 
//! ### Search channel
//! 
//! ```rust
//! use sonic_channel::*;
//! 
//! fn main() -> result::Result<()> {
//!     let channel = SonicChannel::connect_with_start(
//!         ChannelMode::Search,
//!         "localhost:1491",
//!         "SecretPassword",
//!     )?;
//! 
//!     let objects = channel.query("collection", "bucket", "recipe")?;
//!     dbg!(objects);
//! 
//!     Ok(())
//! }
//! ```
//! 
//! ### Ingest channel
//! 
//! ```rust
//! use sonic_channel::*;
//! 
//! fn main() -> result::Result<()> {
//!     let mut channel = SonicChannel::connect_with_start(
//!         ChannelMode::Ingest,
//!         "localhost:1491",
//!         "SecretPassword",
//!     )?;
//! 
//!     let pushed = channel.push("collection", "bucket", "object:1", "my best recipe")?;
//!     // or
//!     // let pushed = channel.push_with_locale("collection", "bucket", "object:1", "Мой лучший рецепт", "rus")?;
//!     dbg!(pushed);
//! 
//!     Ok(())
//! }
//! ```
//! 
//! [sonic]: https://github.com/valeriansaliou/sonic

// Rustc lints.
#![allow(dead_code)]
#![deny(
    missing_debug_implementations,
    unsafe_code,
    unstable_features,
    unused_imports,
    unused_qualifications
)]
#![warn(missing_docs)]

#[cfg(not(any(feature = "ingest", feature = "search", feature = "control")))]
compile_error!(r#"Either features "ingest" or "search" or "control" must be enabled for "sonic-channel" crate"#);

mod channel;
mod commands;

/// Contains sonic channel error type and custom Result type for easy configure your functions.
pub mod result;

pub use channel::*;
pub use commands::*;

#[macro_use]
extern crate lazy_static;
extern crate regex;

#[cfg(test)]
mod tests {
    use crate::channel::ChannelMode;

    #[test]
    fn format_channel_enums() {
        assert_eq!(format!("{}", ChannelMode::Search), String::from("search"));
        assert_eq!(format!("{}", ChannelMode::Ingest), String::from("ingest"));
        assert_eq!(format!("{}", ChannelMode::Control), String::from("control"));
    }

    //TODO: write tests with sonic server
}
