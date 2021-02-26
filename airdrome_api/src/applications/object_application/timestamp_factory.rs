use super::timestamp_model::Timestamp;
use chrono::{NaiveDateTime, SecondsFormat, Utc};

pub fn create_timestamp(timestamp: Option<&str>) -> Result<Timestamp, &'static str> {
    let timestamp = match timestamp {
        Some(t) => t.to_string(),
        None => Utc::now()
            .to_rfc3339_opts(SecondsFormat::Secs, false)
            .replace("+00:00", ""),
    };

    let formatted_timestamp = NaiveDateTime::parse_from_str(&timestamp, "%Y-%m-%dT%H:%M:%S");

    if timestamp.is_empty() {
        Err("Timestamp can't be empty")
    } else if formatted_timestamp.is_err() {
        println!("{}", timestamp);
        Err("Timestamp must in the format yyyy-mm-ddThh:mm:ss")
    } else {
        Timestamp::new(&timestamp)
    }
}
