use anyhow::Result;
use clap::Parser;
use std::path::Path;

#[derive(Parser, Debug)]
pub struct ImageOptions {
    // 输入文件 全路径
    #[clap(short, long, value_parser = verify_file)]
    pub input: String,

    // 压缩比例
    #[clap(short, long)]
    pub quality: u8,

    // 输入文件名
    #[clap(short, long)]
    pub output: Option<String>,

    // 输出文件格式 可以为空 为空时默认和输入文件一致
    #[clap(short, long, value_parser = verify_format)]
    pub format: Option<String>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum ImageType {
    JPG,
    JPEG,
    PNG,
    WEBP,
    UNKNOWN,
}

impl From<&str> for ImageType {
    fn from(image_file: &str) -> Self {
        if let Some(file_type) = image_file.split(".").last() {
            let file_type = match file_type {
                "jpg" => ImageType::JPG,
                "jpeg" => ImageType::JPEG,
                "png" => ImageType::PNG,
                "webp" => ImageType::WEBP,
                _ => ImageType::UNKNOWN,
            };
            file_type
        } else {
            ImageType::UNKNOWN
        }
    }
}

// 检查输出文件的格式是否符合
fn verify_format(format: &str) -> Result<String, &'static str> {
    let file_type: ImageType = format.into();
    if file_type != ImageType::UNKNOWN {
        Ok(format.to_string())
    } else {
        Err("Format not supported")
    }
}

// 检查输入文件的格式是否符合
fn verify_file(file: &str) -> Result<String, String> {
    let file_type: ImageType = file.into();
    if !Path::new(file).exists() {
        Err(format!("File does not exist: {}", file))
    } else if file_type == ImageType::UNKNOWN {
        Err(format!("File format not supported: {}", file))
    } else {
        Ok(file.into())
    }
}
