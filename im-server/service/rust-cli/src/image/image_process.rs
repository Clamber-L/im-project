use crate::image::image_option::ImageOptions;
use anyhow::Result;
use image::codecs::jpeg::JpegEncoder;
use std::fs::File;
use std::io;
use std::io::{Cursor, Read};

pub fn process_image(image_options: ImageOptions) -> Result<()> {
    let format = if let Some(format) = image_options.format {
        format
    } else {
        let split = image_options.input.split(".").last().unwrap();
        split.to_string()
    };

    let mut file = File::open(&image_options.input)?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;
    let memory = image::load_from_memory(&bytes)?;

    let mut output = Cursor::new(Vec::new());
    let mut encoder = JpegEncoder::new_with_quality(&mut output, image_options.quality);
    encoder.encode_image(&memory)?;

    // 保存压缩后的图片
    let out = if image_options.output.is_some() {
        image_options.output.unwrap()
    } else {
        image_options
            .input
            .split("/")
            .last()
            .unwrap()
            .split(".")
            .next()
            .unwrap()
            .to_string()
    };

    let mut file = File::create(format!(
        "compressed_image_{}_{}.{}",
        out, image_options.quality, format
    ))?;
    io::copy(&mut Cursor::new(output.into_inner()), &mut file)?;
    Ok(())
}
