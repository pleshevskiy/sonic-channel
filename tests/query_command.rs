mod common;
use common::*;

const COLLECTION: &str = "Search";
const BUCKET: &str = "Query";

#[test]
fn should_find_object_by_full_text() {
    let title = "Sweet Teriyaki Beef Skewers";

    let ingest_channel = ingest_start();
    ingest_channel.push(COLLECTION, BUCKET, "1", title).unwrap();

    let search_channel = search_start();
    match search_channel.query(COLLECTION, BUCKET, title) {
        Ok(object_ids) => assert_eq!(object_ids, vec!["1"]),
        Err(_) => unreachable!(),
    }

    flush_collection(COLLECTION);
}

#[test]
fn should_find_object_by_parts() {
    let title = "Sweet Teriyaki Beef Skewers";

    let ingest_channel = ingest_start();
    ingest_channel.push(COLLECTION, BUCKET, "1", title).unwrap();

    let search_channel = search_start();

    for word in title.split_whitespace() {
        match search_channel.query(COLLECTION, BUCKET, word) {
            Ok(object_ids) => assert_eq!(object_ids, vec!["1"]),
            Err(_) => unreachable!(),
        }
    }

    flush_collection(COLLECTION);
}

#[test]
fn should_find_many_objects() {
    let ingest_channel = ingest_start();
    ingest_channel
        .push(COLLECTION, BUCKET, "1", "Sweet Teriyaki Beef Skewers")
        .unwrap();
    ingest_channel
        .push(COLLECTION, BUCKET, "2", "Slow Cooker Beef Stew I")
        .unwrap();
    ingest_channel
        .push(COLLECTION, BUCKET, "3", "Christmas Prime Rib")
        .unwrap();

    let search_channel = search_start();
    match search_channel.query(COLLECTION, BUCKET, "Beef") {
        Ok(object_ids) => assert_eq!(object_ids, vec!["2", "1"]),
        Err(_) => unreachable!(),
    }

    flush_collection(COLLECTION);
}
