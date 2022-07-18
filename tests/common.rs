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

pub fn control_start() -> ControlChannel {
    ControlChannel::start(HOST, PASS).expect("The Sonic server must be running")
}

pub fn consolidate() {
    control_start().consolidate().unwrap();
}

pub fn flush_bucket(collection: &str, bucket: &str) {
    ingest_start()
        .flush(FlushRequest::bucket(collection, bucket))
        .unwrap();
}
