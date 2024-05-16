use std::error::Error;
use std::fmt::{Debug, Formatter};
use std::io;

use amqprs::channel::{
    BasicConsumeArguments, BasicPublishArguments, Channel, QueueDeclareArguments,
};
use amqprs::connection::{Connection, OpenConnectionArguments};
use amqprs::BasicProperties;
use log::debug;
use serde::Deserialize;
use serde_json::json;

use crate::clients::response_consumer::ResponseConsumer;
use crate::models::SimplifiedRecItem;

const RPC_QUEUE_RECOMMENDATION_BY_USER: &str = "recommendations.requests.by_user";
const RPC_QUEUE_SET_USER_DESCRIPTION: &str = "resonanse_api.requests.set_user_description";

pub struct RecServiceClient {
    _connection: Connection,
    channel: Channel,
}

impl Debug for RecServiceClient {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "RecServiceClient")
    }
}

impl RecServiceClient {
    pub async fn new(host: &str) -> Self {
        let mut connection_args = OpenConnectionArguments::default();
        connection_args.host(host);

        let connection = Connection::open(&connection_args).await.unwrap();
        let channel = connection.open_channel(None).await.unwrap();

        // declare API queues
        for ms_queue_name in [
            RPC_QUEUE_RECOMMENDATION_BY_USER,
            RPC_QUEUE_SET_USER_DESCRIPTION,
        ] {
            let mut queue_declare_args = QueueDeclareArguments::new(ms_queue_name);
            queue_declare_args.durable(true);
            channel.queue_declare(queue_declare_args).await.unwrap();
        }

        RecServiceClient {
            _connection: connection,
            channel,
        }
    }

    pub async fn rpc_get_recommendation_by_user(
        &self,
        user_id: i64,
    ) -> Result<Vec<SimplifiedRecItem>, Box<dyn Error + Send + Sync>> {
        let request = json!({
            "user_id": user_id,
        })
        .to_string();
        let response = self
            .rpc_call(RPC_QUEUE_RECOMMENDATION_BY_USER, &request)
            .await?;

        let recommendation_response: Vec<SimplifiedRecItem> = serde_json::from_slice(&response)?;
        Ok(recommendation_response)
    }

    pub async fn rpc_set_user_description(
        &self,
        user_id: i64,
        description: &str,
    ) -> Result<bool, Box<dyn Error + Send + Sync>> {
        let request = json!({
            "user_id": user_id,
            "description": description,
        })
        .to_string();
        let response = self
            .rpc_call(RPC_QUEUE_SET_USER_DESCRIPTION, &request)
            .await?;

        #[derive(Deserialize, Debug)]
        struct SetDescriptionResponse {
            status: bool,
        }

        let set_description_response: SetDescriptionResponse = serde_json::from_slice(&response)?;
        debug!("set_description_response is {:?}", set_description_response);
        Ok(set_description_response.status)
    }

    async fn rpc_call(
        &self,
        api_queue_name: &str,
        payload: &str,
    ) -> Result<Vec<u8>, Box<dyn Error + Send + Sync>> {
        let corr_id = uuid::Uuid::new_v4().to_string();

        debug!(
            "send rpc request {:?} corr_id {:?}, payload {:?}",
            api_queue_name, corr_id, payload
        );
        let channel = &self.channel;

        // todo: declaring exclusive queue for every rpc call is bad pattern
        let queue_declare_args = QueueDeclareArguments::exclusive_server_named();
        let exclusive_queue = match channel.queue_declare(queue_declare_args).await? {
            None => {
                return Err(Box::new(io::Error::new(
                    io::ErrorKind::Interrupted,
                    "Cannot declare queue for rpc",
                )))
            }
            Some((queue_name, _, _)) => queue_name,
        };

        // todo: move consumer and response queue to struct level
        let consume_args = BasicConsumeArguments::new(&exclusive_queue, &corr_id);
        let consumer = ResponseConsumer::new();
        let _consumer_tag = channel
            .basic_consume(consumer.clone(), consume_args)
            .await
            .unwrap();

        let properties = BasicProperties::default()
            .with_correlation_id(&corr_id)
            .with_reply_to(&exclusive_queue)
            .finish();
        let pub_args = BasicPublishArguments::new("", api_queue_name);
        channel
            .basic_publish(properties, payload.into(), pub_args)
            .await
            .unwrap();

        let response = consumer.await_response(corr_id).await;
        Ok(response)
    }
}
