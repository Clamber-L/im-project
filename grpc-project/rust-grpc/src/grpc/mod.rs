use std::future::Future;
use std::str::FromStr;
use bb8::ManageConnection;
use tonic::{Request, Status};
use tonic::codegen::InterceptedService;
use tonic::metadata::MetadataValue;
use tonic::service::Interceptor;
use tonic::transport::{Channel, Endpoint};
use crate::pb::user::user_service_client::UserServiceClient;

#[derive(Clone)]
pub struct AuthInterceptor {
    token: String,
}

impl Interceptor for AuthInterceptor {
    fn call(&mut self, mut request: Request<()>) -> Result<Request<()>, Status> {
        let meta_token = MetadataValue::from_str(&self.token).unwrap();
        request.metadata_mut().insert("authorization", meta_token);
        Ok(request)
    }
}

#[derive(Clone)]
pub struct GrpcManager {
    pub endpoint: Endpoint,
    pub token: String,
}

impl ManageConnection for GrpcManager {
    type Connection = UserServiceClient<InterceptedService<Channel, AuthInterceptor>>;
    type Error = tonic::transport::Error;

    fn connect(&self) -> impl Future<Output = Result<<Self as ManageConnection>::Connection, <Self as ManageConnection>::Error>> + Send {
        let endpoint = self.endpoint.clone();
        let token = self.token.clone();

        Box::pin(async move {
            let channel = endpoint.connect().await?;
            let interceptor = AuthInterceptor { token };
            Ok(UserServiceClient::with_interceptor(channel, interceptor))
        })
    }

    fn is_valid(&self, _conn: &mut Self::Connection) -> impl Future<Output = Result<(), <Self as ManageConnection>::Error>> + Send {
        Box::pin(async { Ok(()) })
    }

    fn has_broken(&self, _conn: &mut Self::Connection) -> bool {
        false
    }
}