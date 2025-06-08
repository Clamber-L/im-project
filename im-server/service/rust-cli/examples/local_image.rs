use image::codecs::jpeg::JpegEncoder;
use std::fs::File;
use std::io;
use std::io::{Cursor, Read};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("/Users/lishaowen/Downloads/3.png").unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();

    let img = image::load_from_memory(&buf)?;

    let mut output = Cursor::new(Vec::new());
    // 压缩图片
    let mut encoder = JpegEncoder::new_with_quality(&mut output, 10);
    encoder.encode_image(&img)?;

    // 保存压缩后的图片
    let mut file = File::create("compressed_image_avatar.jpg")?;
    io::copy(&mut Cursor::new(output.into_inner()), &mut file)?;
    Ok(())
}
