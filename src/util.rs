use chrono::{DateTime, TimeZone, Utc};

pub fn now() -> DateTime<Utc> {
    let timestamp:i64 = js_sys::Date::now().floor() as i64;
    chrono::Utc.timestamp_millis(timestamp)
}