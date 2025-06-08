use anyhow::Result;
use clap::Parser;
use rust_cli::chat::chat_process::chat_process;
use rust_cli::csv::csv_process::process_csv;
use rust_cli::image::image_process::process_image;
use rust_cli::time::time_process::process_time;
use rust_cli::{CliOptions, Subcommand};

#[tokio::main]
async fn main() -> Result<()> {
    let parse = CliOptions::parse();
    match parse.cmd {
        Subcommand::Csv(csv) => {
            let output = if let Some(output) = csv.output {
                output.clone()
            } else {
                format!("output.{:?}", csv.format)
            };
            process_csv(&csv.input, output.as_str(), csv.format)?
        }
        Subcommand::ImageQuality(quality) => process_image(quality)?,
        Subcommand::TimeFormat(time_option) => process_time(time_option)?,
        Subcommand::Chat => chat_process().await?,
    }
    Ok(())
}
