use std::{
    fs::File,
    io::{self, Cursor},
};

use image::codecs::jpeg::JpegEncoder;
use reqwest::blocking::get;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let urls = vec![
        (
            "林洪寿",
            "https://jmzbase.oss-cn-beijing.aliyuncs.com/tqrz7tfzty180o2vfis3.JPG",
        ),
        (
            "张维朋",
            "https://jmzbase.oss-cn-beijing.aliyuncs.com/4jaq2p1ms0hv7h0y6u02.JPG",
        ),
        (
            "徐节三",
            "https://jmzbase.oss-cn-beijing.aliyuncs.com/u73gymjvdesib5hz7js8.JPG",
        ),
        (
            "董连财",
            "https://jmzbase.oss-cn-beijing.aliyuncs.com/espj8n07tyb7u7dtw6aq.JPG",
        ),
        (
            "吴振明",
            "https://jmzbase.oss-cn-beijing.aliyuncs.com/bshvu0zgmtekahiuir43.JPG",
        ),
        (
            "李加胜",
            "https://jmzbase.oss-cn-beijing.aliyuncs.com/93ny8nl08t8paoz32gw7.JPG",
        ),
        (
            "孙念泽",
            "https://jmzbase.oss-cn-beijing.aliyuncs.com/cw0bhlmrnwcawi7m7los.jpg",
        ),
        (
            "周维明",
            "https://jmzbase.oss-cn-beijing.aliyuncs.com/w4r35o5edfd1swxli3mi.jpg",
        ),
        (
            "卢林元",
            "https://jmzbase.oss-cn-beijing.aliyuncs.com/rpgmvbdro5czw2m9gnx4.jpg",
        ),
        (
            "吴承志",
            "https://jmzbase.oss-cn-beijing.aliyuncs.com/gik773b3tnh3zdle84ql.jpg",
        ),
        (
            "胡汉武",
            "https://jmzbase.oss-cn-beijing.aliyuncs.com/ouxeqqfrv1yu5isgrkbj.jpg",
        ),
        (
            "李俊谦",
            "https://jmzbase.oss-cn-beijing.aliyuncs.com/519k3hz6v2gw28lxr45x.jpg",
        ),
        (
            "刘洪凯",
            "https://jmzbase.oss-cn-beijing.aliyuncs.com/d76vu1w9grel4rg7wnnp.jpg",
        ),
        (
            "孟德恕",
            "https://jmzbase.oss-cn-beijing.aliyuncs.com/whlxrkzyxo2x5qsbzhkg.JPG",
        ),
        (
            "冯金安",
            "https://jmzbase.oss-cn-beijing.aliyuncs.com/4v1b02zqk17cqshorlok.jpg",
        ),
        (
            "赖建安",
            "https://jmzbase.oss-cn-beijing.aliyuncs.com/0alpa7ya69e7sz04l04t.JPG",
        ),
        (
            "虞国伟",
            "https://jmzbase.oss-cn-beijing.aliyuncs.com/od8pxom4sdjkpwh4w6r5.JPG",
        ),
        (
            "胡高团",
            "https://jmzbase.oss-cn-beijing.aliyuncs.com/0ztpkizudobf2yv6k97n.jpg",
        ),
        (
            "李世雄",
            "https://jmzbase.oss-cn-beijing.aliyuncs.com/qeoic2eaajfynz2us6qq.JPG",
        ),
        (
            "李春宏",
            "https://jmzbase.oss-cn-beijing.aliyuncs.com/ysmgyextfyhg1kc1pzdu.jpg",
        ),
        (
            "倪建玉",
            "https://jmzbase.oss-cn-beijing.aliyuncs.com/g17y8knbn7m0f1d9oayp.JPG",
        ),
        (
            "许纯谊",
            "https://jmzbase.oss-cn-beijing.aliyuncs.com/lhyv6jnguttk1sjbm5pn.jpg",
        ),
        (
            "徐国岩",
            "https://jmzbase.oss-cn-beijing.aliyuncs.com/6w90jlz7wnibuwqzodkv.JPG",
        ),
        (
            "蒋勇",
            "https://jmzbase.oss-cn-beijing.aliyuncs.com/5nk8ps5yxv33uyknpxjo.jpg",
        ),
        (
            "孙祖香",
            "https://jmzbase.oss-cn-beijing.aliyuncs.com/tz0yc5bv1yai8wyz4m3a.JPG",
        ),
        (
            "周绪明",
            "https://jmzbase.oss-cn-beijing.aliyuncs.com/4z99640d5j8d9moc1elo.jpg",
        ),
        (
            "胡树民",
            "https://jmzbase.oss-cn-beijing.aliyuncs.com/aommh6bj5hbofcf169v9.jpeg",
        ),
        (
            "张春霖",
            "https://jmzbase.oss-cn-beijing.aliyuncs.com/vs4f2vx8rumjkww9wjrc.jpg",
        ),
        (
            "陈宏举",
            "https://jmzbase.oss-cn-beijing.aliyuncs.com/vs4f2vx8rumjkww9wjrc.jpg",
        ),
        (
            "梅锦煜",
            "https://jmzbase.oss-cn-beijing.aliyuncs.com/g86w5mxmtwijp4z2yizr.JPG",
        ),
        (
            "高道权",
            "https://jmzbase.oss-cn-beijing.aliyuncs.com/k0oxygtesp4gl9xurtfx.jpg",
        ),
        (
            "交班室",
            "https://jmzbase.oss-cn-beijing.aliyuncs.com/vs4f2vx8rumjkww9wjrc.jpg",
        ),
        (
            "位置2",
            "https://jmzbase.oss-cn-beijing.aliyuncs.com/vs4f2vx8rumjkww9wjrc.jpg",
        ),
        (
            "位置3",
            "https://jmzbase.oss-cn-beijing.aliyuncs.com/vs4f2vx8rumjkww9wjrc.jpg",
        ),
        (
            "图书室",
            "https://jmzbase.oss-cn-beijing.aliyuncs.com/vs4f2vx8rumjkww9wjrc.jpg",
        ),
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
