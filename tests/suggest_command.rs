mod common;
use common::*;

const COLLECTION: &str = "Search";

#[test]
fn should_suggest_nearest_word() {
    let bucket = "suggest_nearest";
    let title = "Sweet Teriyaki Beef Skewers";

    let dest = Dest::col_buc(COLLECTION, bucket);

    let ingest_channel = ingest_start();
    ingest_channel
        .push(PushRequest::new(dest.clone().obj("1"), title))
        .unwrap();

    consolidate();

    let pairs = [
        ("Sweat", "sweet"),
        ("teriaki", "teriyaki"),
        ("Beff", "beef"),
        ("skwers", "skewers"),
    ];

    let search_channel = search_start();
    for (input, expected) in pairs {
        match search_channel.suggest(SuggestRequest::new(dest.clone(), input)) {
            Ok(object_ids) => assert_eq!(object_ids, vec![expected]),
            Err(_) => unreachable!(),
        }
    }

    flush_bucket(COLLECTION, bucket);
}
