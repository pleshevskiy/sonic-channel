mod common;
use common::*;

const COLLECTION: &str = "Ingest";

#[test]
fn should_push_new_object_to_sonic() {
    let bucket = "push_simple";

    let dest = Dest::col_buc(COLLECTION, bucket);

    let ingest_channel = ingest_start();
    match ingest_channel.push(PushRequest::new(
        dest.obj("1"),
        "Sweet Teriyaki Beef Skewers",
    )) {
        Ok(()) => {}
        _ => unreachable!(),
    }

    flush_bucket(COLLECTION, bucket);
}

#[test]
fn should_push_new_object_to_sonic_with_russian_locale() {
    let bucket = "push_locale";

    let dest = Dest::col_buc(COLLECTION, bucket);

    let ingest_channel = ingest_start();
    match ingest_channel.push(
        PushRequest::new(dest.obj("1"), "Открытый пирог с орехами и сгущенкой").lang(Lang::Rus),
    ) {
        Ok(()) => {}
        _ => unreachable!(),
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

    let dest = Dest::col_buc(COLLECTION, bucket);

    let ingest_channel = ingest_start();
    match ingest_channel.push(PushRequest::new(dest.obj("1"), multiline_text)) {
        Ok(()) => {}
        _ => unreachable!(),
    }

    flush_bucket(COLLECTION, bucket);
}
