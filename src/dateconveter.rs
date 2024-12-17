use chrono::{NaiveDateTime, Utc};

pub fn from_unix_to_datetime(unix_seconds: i64) -> String {
    let naive_datetime = NaiveDateTime::from_timestamp_opt(unix_seconds, 0)
        .unwrap();
    let datetime_utc = chrono::DateTime::<Utc>::from_utc(naive_datetime, Utc);
    datetime_utc.to_rfc2822()
}