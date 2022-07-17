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
        .push(PushRequest {
            dest: dest.clone().obj("1"),
            text: title,
            lang: None,
        })
        .unwrap();

    let search_channel = search_start();
    match search_channel.query(QueryRequest {
        dest,
        terms: title,
        lang: None,
    }) {
        Ok(object_ids) => assert_eq!(object_ids, vec![String::from("1")]),
        Err(_) => unreachable!(),
    }

    flush_bucket(COLLECTION, bucket);
}

/*
#[test]
fn should_find_object_by_partial_match() {
    let bucket = "query_by_partial_match";

    let ingest_channel = ingest_start();
    ingest_channel
        .push(COLLECTION, bucket, "1", "Sweet Teriyaki Beef Skewers")
        .unwrap();

    let search_channel = search_start();

    let words = ["Sweet", "Teriyaki", "Beef", "Skewers"];
    for word in words {
        match search_channel.query(COLLECTION, bucket, word) {
            Ok(object_ids) => assert_eq!(object_ids, vec![String::from("1")]),
            Err(_) => unreachable!(),
        }
    }

    flush_bucket(COLLECTION, bucket);
}
*/

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
        .push(PushRequest {
            dest: dest.clone().obj("1"),
            text: multiline_text,
            lang: None,
        })
        .unwrap();

    let words = ["Sweet", "Teriyaki", "Beef", "Skewers"];
    let search_channel = search_start();
    for word in words {
        match search_channel.query(QueryRequest {
            dest: dest.clone(),
            terms: word,
            lang: None,
        }) {
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
        .push(PushRequest {
            dest: dest.clone().obj("1"),
            text: "Sweet Teriyaki Beef Skewers",
            lang: None,
        })
        .unwrap();
    ingest_channel
        .push(PushRequest {
            dest: dest.clone().obj("2"),
            text: "Slow Cooker Beef Stew I",
            lang: None,
        })
        .unwrap();
    ingest_channel
        .push(PushRequest {
            dest: dest.clone().obj("2"),
            text: "Christmas Prime Rib",
            lang: None,
        })
        .unwrap();

    let search_channel = search_start();
    match search_channel.query(QueryRequest {
        dest,
        terms: "Beef",
        lang: None,
    }) {
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
        .push(PushRequest {
            dest: dest.clone().obj("1"),
            text: "Sweet Teriyaki Beef Skewers",
            lang: None,
        })
        .unwrap();
    ingest_channel
        .push(PushRequest {
            dest: dest.clone().obj("2"),
            text: "Slow Cooker Beef Stew I",
            lang: None,
        })
        .unwrap();
    ingest_channel
        .push(PushRequest {
            dest: dest.clone().obj("2"),
            text: "Christmas Prime Rib",
            lang: None,
        })
        .unwrap();

    let search_channel = search_start();
    match search_channel.pag_query(PagQueryRequest {
        dest: dest.clone(),
        terms: "Beef",
        lang: None,
        limit: Some(1),
        offset: None,
    }) {
        Ok(object_ids) => assert_eq!(object_ids, vec!["2"]),
        Err(_) => unreachable!(),
    }

    let search_channel = search_start();
    match search_channel.pag_query(PagQueryRequest {
        dest,
        terms: "Beef",
        lang: None,
        limit: Some(1),
        offset: Some(1),
    }) {
        Ok(object_ids) => assert_eq!(object_ids, vec!["1"]),
        Err(_) => unreachable!(),
    }

    flush_bucket(COLLECTION, bucket);
}
