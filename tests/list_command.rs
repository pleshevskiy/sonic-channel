mod common;
use common::*;

const COLLECTION: &str = "Search";

#[test]
fn should_list_all_words() {
    let bucket = "suggest_nearest";
    let title = "Sweet Teriyaki Beef Skewers";

    let dest = Dest::col_buc(COLLECTION, bucket);

    let ingest_channel = ingest_start();
    ingest_channel
        .push(PushRequest::new(dest.clone().obj("1"), title))
        .unwrap();

    consolidate();

    let search_channel = search_start();
    match search_channel.list(ListRequest::new(dest.clone())) {
        Ok(object_ids) => assert_eq!(object_ids, vec!["beef", "skewers", "sweet", "teriyaki"]),
        Err(e) => unreachable!(),
    }

    flush_bucket(COLLECTION, bucket);
}
