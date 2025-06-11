use chrono::{DateTime, Datelike, Duration, TimeZone, Utc};

pub fn today_date() -> (DateTime<Utc>, DateTime<Utc>) {
    let now = Utc::now();
    let today_start = Utc
        .with_ymd_and_hms(now.year(), now.month(), now.day(), 0, 0, 0)
        .unwrap();
    let today_end = today_start + Duration::days(1);
    (today_start, today_end)
}
