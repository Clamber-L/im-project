mod grpc;
mod logger;
mod pb;
mod response_macro;

use crate::grpc::GrpcManager;
use crate::logger::init_logger;
use crate::pb::user::GetUserRequest;
use anyhow::Result;
use bb8::Pool;
use tonic::transport::Endpoint;
use tonic::Request;
use tracing::{error, info, warn};

#[tokio::main]
async fn main() -> Result<()> {
    // 日志
    let _guard = init_logger("rust-grpc-project");
    info!("starting grpc server");
    // 初始化连接池
    let endpoint = Endpoint::from_static("http://localhost:8999");
    let manager = GrpcManager {
        endpoint,
        token: "".to_string(),
    };
    warn!("starting grpc server");
    let pool = Pool::builder().max_size(10).build(manager).await?;

    error!("pool started");
    let mut client = pool.get().await?;
    let request = Request::new(GetUserRequest { id: 12_u64 });

    let user = client.get_user(request).await;
    info!("用户信息：{:?}", user);
    Ok(())
}
