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
        .push(PushRequest {
            dest: dest.clone().obj("1"),
            text: title,
            lang: None,
        })
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
        match search_channel.suggest(SuggestRequest {
            dest: dest.clone(),
            word: input,
        }) {
            Ok(object_ids) => assert_eq!(object_ids, vec![expected]),
            Err(_) => unreachable!(),
        }
    }

    flush_bucket(COLLECTION, bucket);
}
