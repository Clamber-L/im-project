use crate::pb::message::message_service_client::MessageServiceClient;
use bb8::ManageConnection;
use std::future::Future;
use tonic::transport::{Channel, Endpoint};

#[derive(Clone)]
pub struct MessageGrpcManager {
    pub endpoint: Endpoint,
    pub token: String,
}

impl ManageConnection for MessageGrpcManager {
    type Connection = MessageServiceClient<Channel>;
    type Error = tonic::transport::Error;

    fn connect(&self) -> impl Future<Output = Result<Self::Connection, Self::Error>> + Send {
        let endpoint = self.endpoint.clone();

        Box::pin(async move {
            let channel = endpoint.connect().await?;
            Ok(MessageServiceClient::new(channel))
        })
    }

    fn is_valid(
        &self,
        _conn: &mut Self::Connection,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send {
        //当你通过 pool.get().await 拿一个连接出来时，bb8 会调用这个方法来判断这个连接是否仍然可用。
        // 🧠 示例应用场景：
        // 数据库连接是否还活着（执行一个 SELECT 1）
        // gRPC 通道是否还能成功发送心跳
        // Redis/Pg/Mysql 等连接常常会在这里跑一个 PING/PONG 测试
        // 连接是 tonic 的 Channel，其健康性一般可以用失败自动恢复 + retry 处理
        Box::pin(async { Ok(()) })
    }

    fn has_broken(&self, _conn: &mut Self::Connection) -> bool {
        //在连接用完归还池时，bb8 会调用这个方法来判断：
        // “这个连接还能回池继续用吗，还是彻底坏了该丢掉？”
        // 🧠 应用场景：
        // 数据库连接池中 socket 被对端关闭（你需要判断 conn 是否已失效）
        // gRPC 客户端连接已 drop（连接不可再用）
        // ❗ Rust 的 tonic Channel 本身不会暴露“是否已挂”的状态，除非你内部维护状态。
        // 所以一般你只能简单返回 false，让 bb8 认为连接始终“没有彻底损坏”。
        false
    }
}
