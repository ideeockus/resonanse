use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Debug, Formatter};
use std::sync::{Arc, Mutex};

use amqprs::{AmqpMessageCount, BasicProperties, Deliver};
use amqprs::callbacks::{DefaultChannelCallback, DefaultConnectionCallback};
use amqprs::channel::{BasicConsumeArguments, BasicPublishArguments, Channel, QueueDeclareArguments};
use amqprs::connection::{Connection, OpenConnectionArguments};
use amqprs::consumer::{AsyncConsumer, DefaultConsumer};
use log::debug;
use serde_json::{json, Value};
use crate::clients::error::RpcError;

use crate::clients::response_consumer::ResponseConsumer;
use crate::RecItem;

const RPC_QUEUE_RECOMMENDATION_BY_USER: &str = "recommendations.requests.by_user";
const RPC_QUEUE_SET_USER_DESCRIPTION: &str = "resonanse_api.requests.set_user_description";


pub struct RecServiceClient {
    connection: Arc<Mutex<Connection>>,
    channel: Arc<Mutex<Channel>>,
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
            let queue_declare_args = QueueDeclareArguments::new(ms_queue_name);
            channel.queue_declare(queue_declare_args).await.unwrap();
        }

        RecServiceClient {
            connection: Arc::new(Mutex::new(connection)),
            channel: Arc::new(Mutex::new(channel)),
        }
    }

    pub async fn rpc_get_recommendation_by_user(&self, user_id: i64) -> Result<Option<Vec<RecItem>>, Box<dyn Error + Send + Sync>> {
        let request = json!({
            "user_id": user_id,
        }).to_string();
        let response = match self.rpc_call(RPC_QUEUE_RECOMMENDATION_BY_USER, &request).await? {
            None => return Ok(None),
            Some(v) => v,
        };

        let recommendation_response: Vec<RecItem> = serde_json::from_slice(&response)?;
        Ok(Some(recommendation_response))
    }

    pub async fn rpc_set_user_description(
        &self,
        user_id: i64,
        description: &str,
    ) -> Result<bool, Box<dyn Error + Send + Sync>> {
        let request = json!({
            "user_id": user_id,
            "description": description,
        }).to_string();
        let response = match self.rpc_call(RPC_QUEUE_SET_USER_DESCRIPTION, &request).await? {
            None => return Ok(false),
            Some(v) => v,
        };

        let set_description_response: HashMap<String, String> = serde_json::from_slice(&response)?;
        debug!("set_description_response is {:?}", set_description_response);
        // todo: is not always true !
        Ok(true)
    }

    async fn rpc_call(&self, api_queue_name: &str, payload: &str) -> Result<Option<Vec<u8>>, Box<dyn Error + Send + Sync>> {
        let corr_id = uuid::Uuid::new_v4().to_string();

        debug!("send rpc request {:?} corr_id {:?}, payload {:?}", api_queue_name, corr_id, payload);
        let channel = self.channel.lock().unwrap();


        // todo: declaring exclusive queue for every rpc call is bad pattern
        let queue_declare_args = QueueDeclareArguments::exclusive_server_named();
        let exclusive_queue = match channel.queue_declare(queue_declare_args).await? {
            None => return Ok(None),
            Some((queue_name, _, _)) => queue_name,
        };

        // todo: move consumer and response queue to struct level
        let consume_args = BasicConsumeArguments::new(
            &exclusive_queue,
            &corr_id,
        );
        let consumer = ResponseConsumer::new();
        let _consumer_tag = channel
            .basic_consume(consumer.clone(), consume_args)
            .await
            .unwrap();

        let properties = BasicProperties::default()
            .with_correlation_id(&corr_id)
            .with_reply_to(&exclusive_queue)
            .finish();
        let pub_args = BasicPublishArguments::new(
            "",
            api_queue_name,
        );
        channel
            .basic_publish(
                properties,
                payload.into(),
                pub_args,
            )
            .await
            .unwrap();

        let response = consumer.await_response(corr_id).await;
        Ok(Some(response))
    }
}