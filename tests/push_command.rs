mod common;
use common::*;

const COLLECTION: &str = "Ingest";
const BUCKET: &str = "Push";

#[test]
fn should_push_new_object_to_sonic() {
    let ingest_channel = ingest_start();

    match ingest_channel.push(COLLECTION, BUCKET, "1", "Sweet Teriyaki Beef Skewers") {
        Ok(res) => assert!(res),
        Err(_) => unreachable!(),
    }

    flush_collection(COLLECTION);
}

#[test]
fn should_push_new_object_to_sonic_with_russian_locale() {
    let ingest_channel = ingest_start();

    match ingest_channel.push_with_locale(
        COLLECTION,
        BUCKET,
        "1",
        "Открытый пирог с орехами и сгущенкой",
        "rus",
    ) {
        Ok(res) => assert!(res),
        Err(_) => unreachable!(),
    }

    flush_collection(COLLECTION);
}
