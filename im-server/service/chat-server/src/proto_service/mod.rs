use std::ops::DerefMut;

use axum::extract::ws::Message as WsMessage;
use futures_util::SinkExt;
use lib_rpc::pb::message::{message_service_server::MessageService, Message, SendMessageResponse};
use serde_json::json;
use tonic::{Request, Response, Status};

use crate::ConnectionMap;

type ServiceResponse<T> = Result<Response<T>, Status>;

#[derive(Default)]
pub struct MessageGrpc {
    pub connection_map: ConnectionMap,
}

#[tonic::async_trait]
impl MessageService for MessageGrpc {
    async fn send_message(
        &self,
        request: Request<Message>,
    ) -> ServiceResponse<SendMessageResponse> {
        let message = request.into_inner();
        if let Some(mut connection_id) = self.connection_map.clone().get_mut(&message.to_user_id) {
            connection_id
                .deref_mut()
                .send(WsMessage::Text(json!(message).to_string().into()))
                .await
                .unwrap();
        }

        Ok(Response::new(SendMessageResponse::default()))
    }
}
