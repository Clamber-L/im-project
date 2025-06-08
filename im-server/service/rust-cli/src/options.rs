use crate::csv::csv_option::CsvOptions;
use crate::image::image_option::ImageOptions;
use crate::time::time_option::TimeOptions;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "cli", version, author, about, long_about = None)]
pub struct CliOptions {
    #[command(subcommand)]
    pub cmd: Subcommand,
}

#[derive(Debug, Parser)]
pub enum Subcommand {
    // cli csv -i input.csv -o output.json
    #[command(name = "csv", about = "把csv转化为json")]
    Csv(CsvOptions),

    // cli image -i input.jpg -q 80 -o output.png
    #[command(name = "image", about = "图片分辨率压缩")]
    ImageQuality(ImageOptions),

    #[command(name = "time", about = "获取时间戳")]
    TimeFormat(TimeOptions),

    #[command(name = "chat", about = "web socket")]
    Chat,
}
