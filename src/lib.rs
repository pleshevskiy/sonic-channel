//! # Sonic Channel
//! Rust client for [sonic] search backend.
//!
//!
//! ## Example usage
//!
//! ### Search channel
//!
//! Note: This example requires enabling the `search` feature, enabled by default.
//!
//! ```rust,no_run
//! use sonic_channel::*;
//!
//! fn main() -> result::Result<()> {
//!     let channel = SearchChannel::start(
//!         "localhost:1491",
//!         "SecretPassword",
//!     )?;
//!
//!     let objects = channel.query(QueryRequest {
//!         dest: Dest::col_buc("collection", "bucket"),
//!         terms: "recipe",
//!         lang: None,
//!     })?;
//!     dbg!(objects);
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Ingest channel
//!
//! Note: This example requires enabling the `ingest` feature.
//!
//! ```rust,no_run
//! use sonic_channel::*;
//!
//! fn main() -> result::Result<()> {
//!     let mut channel = IngestChannel::start(
//!         "localhost:1491",
//!         "SecretPassword",
//!     )?;
//!
//!     let pushed = channel.push(PushRequest {
//!         dest: Dest::col_buc("collection", "bucket").obj("object:1"),
//!         text: "my best recipe",
//!         lang: None, // <<< auto detect
//!     })?;
//!     // or
//!     // let pushed = channel.push(PushRequest {
//!     //     dest: Dest::col_buc("collection", "bucket").obj("object:1"),
//!     //     text: "Мой лучший рецепт",
//!     //     lang: Some(Lang::Rus)
//!     // })?;
//!     dbg!(pushed);
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Control channel
//!
//! Note: This example requires enabling the `control` feature.
//!
//! ```rust,no_run
//! use sonic_channel::*;
//!
//! fn main() -> result::Result<()> {
//!     let mut channel = ControlChannel::start(
//!         "localhost:1491",
//!         "SecretPassword",
//!     )?;
//!
//!     let result = channel.consolidate()?;
//!     assert_eq!(result, ());
//!
//!     Ok(())
//! }
//! ```
//!
//! [sonic]: https://github.com/valeriansaliou/sonic

// Rustc lints.
#![deny(
    missing_debug_implementations,
    unsafe_code,
    unstable_features,
    unused_imports,
    unused_qualifications
)]
#![warn(missing_docs)]
// Clippy lints
#![deny(clippy::all)]

#[cfg(not(any(feature = "ingest", feature = "search", feature = "control")))]
compile_error!(
    r#"Either features "ingest" or "search" or "control" must be enabled for "sonic-channel" crate"#
);

#[macro_use]
mod macroses;

pub(crate) mod protocol;

mod channels;
mod commands;

mod misc;

/// Contains sonic channel error type and custom Result type for easy configure your functions.
pub mod result;

pub use channels::*;
pub use commands::*;
pub use misc::*;

pub use whatlang::Lang;
