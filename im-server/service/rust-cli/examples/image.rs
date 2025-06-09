use std::{
    fs::File,
    io::{self, Cursor},
};

use image::codecs::jpeg::JpegEncoder;
use reqwest::blocking::get;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let urls = vec![
        (
            "棋牌室",
            "https://jmzbase.oss-cn-beijing.aliyuncs.com/vs4f2vx8rumjkww9wjrc.jpg",
        ),
    ];
    for (_, item) in urls.into_iter().enumerate() {
        match download_and_compress(item) {
            Ok(_) => println!("Image downloaded and compressed successfully!"),
            Err(e) => eprintln!("Error processing image {}: {}", item.0, e),
        }
    }

    Ok(())
}

fn download_and_compress(item: (&str, &str)) -> Result<(), Box<dyn std::error::Error>> {
    let mut response = get(item.1)?;
    let mut buf: Vec<u8> = vec![];
    response.copy_to(&mut buf)?;

    // 读入图片
    let img = image::load_from_memory(&buf)?;

    let mut output = Cursor::new(Vec::new());
    // 压缩图片
    let mut encoder = JpegEncoder::new_with_quality(&mut output, 30);
    encoder.encode_image(&img)?;

    // 保存压缩后的图片
    let filename = format!("E:\\all-jar\\徐汇老干部\\compressed_image_{}.jpg", item.0);
    let mut file = File::create(filename)?;
    io::copy(&mut Cursor::new(output.into_inner()), &mut file)?;

    println!("{} 压缩成功", item.0);
    Ok(())
}
