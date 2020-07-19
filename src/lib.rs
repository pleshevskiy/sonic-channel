#![allow(dead_code)]

mod channel;
mod commands;
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
