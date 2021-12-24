mod common;
use common::*;

const COLLECTION: &str = "Ingest";

#[test]
fn should_push_new_object_to_sonic() {
    let bucket = "push_simple";

    let ingest_channel = ingest_start();
    match ingest_channel.push(COLLECTION, bucket, "1", "Sweet Teriyaki Beef Skewers") {
        Ok(res) => assert!(res),
        Err(_) => unreachable!(),
    }

    flush_bucket(COLLECTION, bucket);
}

#[test]
fn should_push_new_object_to_sonic_with_russian_locale() {
    let bucket = "push_locale";

    let ingest_channel = ingest_start();
    match ingest_channel.push_with_locale(
        COLLECTION,
        bucket,
        "1",
        "Открытый пирог с орехами и сгущенкой",
        "rus",
    ) {
        Ok(res) => assert!(res),
        Err(_) => unreachable!(),
    }

    flush_bucket(COLLECTION, bucket);
}

#[test]
fn should_push_multiline_text() {
    let bucket = "push_multiline";
    let multiline_text = "
Sweet
Teriyaki
Beef
Skewers
";

    let ingest_channel = ingest_start();
    match ingest_channel.push(COLLECTION, bucket, "1", multiline_text) {
        Ok(res) => assert!(res),
        Err(_) => unreachable!(),
    }

    flush_bucket(COLLECTION, bucket);
}
