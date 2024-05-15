use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use amqprs::channel::{BasicAckArguments, Channel};

use amqprs::consumer::AsyncConsumer;
use amqprs::{BasicProperties, Deliver};
use async_trait::async_trait;
use log::debug;
use tokio::sync::Notify;

#[derive(Clone)]
pub struct ResponseConsumer {
    responses: Arc<Mutex<HashMap<String, Vec<u8>>>>,
    notify: Arc<Notify>,
}

impl ResponseConsumer {
    pub fn new() -> Self {
        ResponseConsumer {
            responses: Arc::new(Mutex::new(HashMap::new())),
            notify: Arc::new(Notify::new()),
        }
    }

    pub async fn await_response(&self, corr_id: String) -> Vec<u8> {
        loop {
            {
                let mut responses = self.responses.lock().unwrap();
                if let Some(response) = responses.remove(&corr_id) {
                    return response;
                }
            }
            self.notify.notified().await;
        }
    }
}


#[async_trait]
impl AsyncConsumer for ResponseConsumer {
    async fn consume(
        &mut self,
        _channel: &Channel,
        _deliver: Deliver,
        basic_properties: BasicProperties,
        content: Vec<u8>,
    ) {
        let corr_id = basic_properties.correlation_id().clone();
        let response = content.clone();


        if let Some(corr_id) = corr_id {
            debug!("got rpc response corr_id {:?}", corr_id);
            let mut responses = self.responses.lock().unwrap();
            responses.insert(corr_id.clone(), response);
            self.notify.notify_one();
        }
    }
}


