use futures_util::{SinkExt, StreamExt};
use net_server::DollarCodec;
use std::io;
use tokio::net::TcpStream;
use tokio_util::codec::Framed;

#[tokio::main]
async fn main() -> io::Result<()> {
    // 创建 TCP 连接
    let stream = TcpStream::connect("127.0.0.1:8080").await?;

    // 使用 Framed 和自定义的 DollarCodec
    let mut framed = Framed::new(stream, DollarCodec);

    // 发送消息
    framed
        .send("S168#864239068739969#8a36#000a#RET,JUST,1".to_string())
        .await?;

    // 接收消息
    while let Some(result) = framed.next().await {
        match result {
            Ok(msg) => println!("Received: {}", msg),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
    Ok(())
}
