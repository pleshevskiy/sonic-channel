mod common;
use common::*;

const COLLECTION: &str = "Ingest";

#[test]
fn should_push_new_object_to_sonic() {
    let bucket = "push_simple";

    let dest = Dest::col_buc(COLLECTION, bucket);

    let ingest_channel = ingest_start();
    match ingest_channel.push(PushRequest {
        dest: dest.obj("1"),
        text: "Sweet Teriyaki Beef Skewers",
        lang: None,
    }) {
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
    match ingest_channel.push(PushRequest {
        dest: dest.obj("1"),
        text: "Открытый пирог с орехами и сгущенкой",
        lang: Some(Lang::Rus),
    }) {
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
    match ingest_channel.push(PushRequest {
        dest: dest.obj("1"),
        text: multiline_text,
        lang: None,
    }) {
        Ok(()) => {}
        _ => unreachable!(),
    }

    flush_bucket(COLLECTION, bucket);
}
