mod grpc;
mod pb;

use crate::pb::user::GetUserRequest;
use anyhow::Result;
use bb8::Pool;
use tonic::Request;
use tonic::transport::Endpoint;
use crate::grpc::GrpcManager;

#[tokio::main]
async fn main() -> Result<()> {
    // 初始化连接池
    let endpoint = Endpoint::from_static("http://localhost:8999");
    let manager = GrpcManager {
        endpoint,
        token: "1234332".to_string(),
    };
    
    let pool = Pool::builder().max_size(10).build(manager).await?;

    let mut client = pool.get().await?;
    let request = Request::new(GetUserRequest {
        id: 12_u64
    });

    let response = client.get_user(request).await?;
    println!("RESPONSE={:?}", response);
    Ok(())
}
