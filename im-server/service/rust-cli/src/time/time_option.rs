use anyhow::Result;
use clap::Parser;
use std::str::FromStr;

#[derive(Debug, Parser)]
pub struct TimeOptions {
    #[clap(short, long)]
    pub input: Option<String>,

    #[clap(short, long, default_value = "s", value_parser = verify_format)]
    pub format: TimeFormat,
}

#[derive(Debug, Clone)]
pub enum TimeFormat {
    Second,
    Millisecond,
}

pub fn verify_format(format: &str) -> Result<TimeFormat, anyhow::Error> {
    format.parse()
}

impl FromStr for TimeFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "s" => Ok(TimeFormat::Second),
            "ms" => Ok(TimeFormat::Millisecond),
            _ => Err(anyhow::anyhow!("Invalid time format")),
        }
    }
}
