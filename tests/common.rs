#![allow(dead_code)]

pub use sonic_channel::*;

pub const HOST: &str = "localhost:36999";
pub const PASS: &str = "SecretPassword1234";

pub fn ingest_start() -> IngestChannel {
    IngestChannel::start(HOST, PASS).expect("The Sonic server must be running")
}

pub fn search_start() -> SearchChannel {
    SearchChannel::start(HOST, PASS).expect("The Sonic server must be running")
}

pub fn flush_collection(collection: &str) {
    ingest_start().flushc(collection).unwrap();
}
