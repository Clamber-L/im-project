mod grpc;
mod pb;

use crate::pb::user::user_service_client::UserServiceClient;
use crate::pb::user::GetUserRequest;
use anyhow::Result;
use tonic::Request;

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = UserServiceClient::connect("http://192.168.0.20:8999").await?;
    let response = client
        .get_user(Request::new(GetUserRequest { id: 123u64 }))
        .await?;

    println!("RESPONSE={:?}", response.into_inner());
    Ok(())
}
