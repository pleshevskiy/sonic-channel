mod common;
use common::*;

const COLLECTION: &str = "Search";

#[test]
fn should_find_object_by_exact_match() {
    let bucket = "query_by_exact_match";
    let title = "Sweet Teriyaki Beef Skewers";

    let dest = Dest::col_buc(COLLECTION, bucket);

    let ingest_channel = ingest_start();
    ingest_channel
        .push(PushRequest::new(dest.clone().obj("1"), title))
        .unwrap();

    consolidate();

    let search_channel = search_start();
    match search_channel.query(QueryRequest::new(dest, title)) {
        Ok(object_ids) => assert_eq!(object_ids, vec![String::from("1")]),
        Err(_) => unreachable!(),
    }

    flush_bucket(COLLECTION, bucket);
}

#[test]
fn should_find_multiline_object_by_partial_match() {
    let bucket = "query_multiline";
    let multiline_text = "
Sweet
Teriyaki
Beef
Skewers
None";

    let dest = Dest::col_buc(COLLECTION, bucket);

    let ingest_channel = ingest_start();
    ingest_channel
        .push(PushRequest::new(dest.clone().obj("1"), multiline_text))
        .unwrap();

    consolidate();

    let words = ["Sweet", "Teriyaki", "Beef", "Skewers"];
    let search_channel = search_start();
    for word in words {
        match search_channel.query(QueryRequest::new(dest.clone(), word)) {
            Ok(object_ids) => assert_eq!(object_ids, vec![String::from("1")]),
            Err(_) => unreachable!(),
        }
    }

    flush_bucket(COLLECTION, bucket);
}

#[test]
fn should_find_many_objects() {
    let bucket = "query_many_objects";

    let dest = Dest::col_buc(COLLECTION, bucket);

    let ingest_channel = ingest_start();
    ingest_channel
        .push(PushRequest::new(
            dest.clone().obj("1"),
            "Sweet Teriyaki Beef Skewers",
        ))
        .unwrap();
    ingest_channel
        .push(PushRequest::new(
            dest.clone().obj("2"),
            "Slow Cooker Beef Stew I",
        ))
        .unwrap();
    ingest_channel
        .push(PushRequest::new(
            dest.clone().obj("3"),
            "Christmas Prime Rib",
        ))
        .unwrap();

    consolidate();

    let search_channel = search_start();
    match search_channel.query(QueryRequest::new(dest, "Beef")) {
        Ok(object_ids) => assert_eq!(object_ids, vec!["2", "1"]),
        Err(_) => unreachable!(),
    }

    flush_bucket(COLLECTION, bucket);
}

#[test]
fn should_find_limited_objects() {
    let bucket = "query_limited_objects";

    let dest = Dest::col_buc(COLLECTION, bucket);

    let ingest_channel = ingest_start();
    ingest_channel
        .push(PushRequest::new(
            dest.clone().obj("1"),
            "Sweet Teriyaki Beef Skewers",
        ))
        .unwrap();
    ingest_channel
        .push(PushRequest::new(
            dest.clone().obj("2"),
            "Slow Cooker Beef Stew I",
        ))
        .unwrap();
    ingest_channel
        .push(PushRequest::new(
            dest.clone().obj("3"),
            "Christmas Prime Rib",
        ))
        .unwrap();

    consolidate();

    let search_channel = search_start();
    match search_channel.query(QueryRequest::new(dest.clone(), "Beef").limit(1)) {
        Ok(object_ids) => assert_eq!(object_ids, vec!["2"]),
        Err(_) => unreachable!(),
    }

    let search_channel = search_start();
    match search_channel.query(QueryRequest::new(dest, "Beef").pag(1, 1)) {
        Ok(object_ids) => assert_eq!(object_ids, vec!["1"]),
        Err(_) => unreachable!(),
    }

    flush_bucket(COLLECTION, bucket);
}
