use anyhow::Result;
use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio::task;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::tungstenite::Utf8Bytes;

#[tokio::main]
async fn main() -> Result<()> {
    // TCP 监听端口
    let tcp_port = 8080;
    // WebSocket 监听端口
    let websocket_port = 8081;
    // MQTT 监听端口
    let mqtt_port = 1883;

    // 创建 Tokio 任务来同时监听多个端口
    let tcp_listener_task = task::spawn(run_tcp_listener(tcp_port));
    let websocket_listener_task = task::spawn(run_websocket_listener(websocket_port));
    let mqtt_listener_task = task::spawn(run_mqtt_listener(mqtt_port));

    // 等待所有任务完成
    let _res = tokio::try_join!(
        tcp_listener_task,
        websocket_listener_task,
        mqtt_listener_task
    )?;

    Ok(())
}

async fn run_tcp_listener(port: u16) -> Result<()> {
    let listener = TcpListener::bind(("0.0.0.0", port)).await?;
    println!("TCP listener running on port {}", port);

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New TCP connection from {}", addr);

        // 处理 TCP 连接
        task::spawn(async move {
            let mut buf = [0; 1024];
            let socket = socket;

            if let Ok(size) = socket
                .readable()
                .await
                .and_then(|_| socket.try_read(&mut buf))
            {
                println!(
                    "Received from TCP client: {}",
                    String::from_utf8_lossy(&buf[..size])
                );
                let _ = socket
                    .writable()
                    .await
                    .and_then(|_| socket.try_write(&buf[..size]));
            }
        });
    }
}

async fn run_websocket_listener(port: u16) -> Result<()> {
    let listener = TcpListener::bind(("0.0.0.0", port)).await?;
    println!("WebSocket listener running on port {}", port);

    loop {
        let (stream, addr) = listener.accept().await?;
        println!("New WebSocket connection from {}", addr);

        // 处理 WebSocket 连接
        task::spawn(async move {
            let ws_stream = accept_async(stream)
                .await
                .expect("Error during the WebSocket handshake");
            println!("WebSocket handshake successful with {}", addr);

            let (mut write, mut read) = ws_stream.split();

            while let Some(Ok(msg)) = read.next().await {
                println!("Received WebSocket message: {}", msg);
                if msg.is_text() || msg.is_binary() {
                    let message = Message::Text(Utf8Bytes::from_static("Hello from WebSocket"));
                    write.send(message).await.expect("TODO: panic message");
                }
            }
        });
    }
}

async fn run_mqtt_listener(port: u16) -> Result<()> {
    let listener = TcpListener::bind(("0.0.0.0", port)).await?;
    println!("MQTT listener running on port {}", port);

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New MQTT connection from {}", addr);

        // 处理 MQTT 连接（此处为简化示例，没有完整实现 MQTT 协议）
        task::spawn(async move {
            let mut buf = [0; 1024];
            let socket = socket;

            if let Ok(size) = socket
                .readable()
                .await
                .and_then(|_| socket.try_read(&mut buf))
            {
                println!("Received from MQTT client: {:?}", &buf[..size]);
                let _ = socket
                    .writable()
                    .await
                    .and_then(|_| socket.try_write(b"MQTT response"));
            }
        });
    }
}
