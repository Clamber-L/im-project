use bytes::BytesMut;
use std::io;
use tokio_util::codec::{Decoder, Encoder};

// 自定义 Codec
pub struct DollarCodec;

impl Decoder for DollarCodec {
    type Item = String; // 每条解析出的消息
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        // 查找 `$` 作为结束标志
        if let Some(pos) = src.iter().position(|&b| b == b'$') {
            // 分离出完整的一条消息
            let mut line = src.split_to(pos + 1); // 包括 `$`
            line.truncate(line.len() - 1); // 去掉 `$`

            // 转换为字符串
            match String::from_utf8(line.to_vec()) {
                Ok(msg) => Ok(Some(msg)),
                Err(_) => Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8")),
            }
        } else {
            // 如果没有找到 `$`，返回 None 以等待更多数据
            Ok(None)
        }
    }
}

impl Encoder<String> for DollarCodec {
    type Error = io::Error;

    fn encode(&mut self, item: String, dst: &mut BytesMut) -> Result<(), Self::Error> {
        // 将消息加上 `$` 作为结尾
        dst.extend_from_slice(item.as_bytes());
        dst.extend_from_slice(b"$");
        Ok(())
    }
}

