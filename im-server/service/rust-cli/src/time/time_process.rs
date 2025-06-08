use crate::time::time_option::{TimeFormat, TimeOptions};
use anyhow::Result;
use chrono::{NaiveDateTime, Utc};

pub fn process_time(option: TimeOptions) -> Result<()> {
    let time = if let Some(time) = option.input {
        NaiveDateTime::parse_from_str(time.as_str(), "%Y-%m-%d %H:%M:%S")?.and_utc()
    } else {
        Utc::now()
    };

    match option.format {
        TimeFormat::Second => {
            println!("{}", time.timestamp());
        }
        TimeFormat::Millisecond => {
            println!("{}", time.timestamp_millis());
        }
    }
    Ok(())
}
