mod common;
use common::*;

const COLLECTION: &str = "Search";

#[test]
fn should_find_object_by_exact_match() {
    let bucket = "query_by_exact_match";
    let title = "Sweet Teriyaki Beef Skewers";

    let ingest_channel = ingest_start();
    ingest_channel.push(COLLECTION, bucket, "1", title).unwrap();

    let search_channel = search_start();
    match search_channel.query(COLLECTION, bucket, title) {
        Ok(object_ids) => assert_eq!(object_ids, vec!["1"]),
        Err(_) => unreachable!(),
    }

    flush_bucket(COLLECTION, bucket);
}

#[test]
fn should_find_object_by_partial_match() {
    let bucket = "query_by_partial_match";
    let title = "Sweet Teriyaki Beef Skewers";

    let ingest_channel = ingest_start();
    ingest_channel.push(COLLECTION, bucket, "1", title).unwrap();

    let search_channel = search_start();

    for word in title.split_whitespace() {
        match search_channel.query(COLLECTION, bucket, word) {
            Ok(object_ids) => assert_eq!(object_ids, vec!["1"]),
            Err(_) => unreachable!(),
        }
    }

    flush_bucket(COLLECTION, bucket);
}

#[test]
fn should_find_many_objects() {
    let bucket = "query_many_objects";

    let ingest_channel = ingest_start();
    ingest_channel
        .push(COLLECTION, bucket, "1", "Sweet Teriyaki Beef Skewers")
        .unwrap();
    ingest_channel
        .push(COLLECTION, bucket, "2", "Slow Cooker Beef Stew I")
        .unwrap();
    ingest_channel
        .push(COLLECTION, bucket, "3", "Christmas Prime Rib")
        .unwrap();

    let search_channel = search_start();
    match search_channel.query(COLLECTION, bucket, "Beef") {
        Ok(object_ids) => assert_eq!(object_ids, vec!["2", "1"]),
        Err(_) => unreachable!(),
    }

    flush_bucket(COLLECTION, bucket);
}
