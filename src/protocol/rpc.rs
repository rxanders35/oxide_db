use serde::{Deserialize, Serialize};
use std::any::TypeId;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};

#[derive(Serialize, Deserialize)]
pub struct RequestMessage {
    endname: String,
    service_method: String,
    args_type: String,
    args: Vec<u8>,
    #[serde(skip)]
    reply_tx: Option<tokio::sync::oneshot::Sender<ReplyMessage>>,
}

#[derive(Serialize, Deserialize)]
pub struct ReplyMessage {
    success: bool,
    reply: Vec<u8>,
}

struct ClientEnd {
    endname: String,
    channel: mpsc::Sender<RequestMessage>,
    done: Arc<Mutex<bool>>,
}

impl ClientEnd {
    pub fn new(
        endname: String,
        channel: mpsc::Sender<RequestMessage>,
        done: Arc<Mutex<bool>>,
    ) -> Self {
        ClientEnd {
            endname,
            channel,
            done,
        }
    }
    pub async fn call<T: Serialize, R: for<'de> Deserialize<'de>>(
        &self,
        service_method: String,
        args: T,
        reply: &mut R,
    ) -> bool {
        let bytes = match bincode::serialize(&args) {
            Ok(b) => b,
            Err(err) => panic!("Failed to serialize args: {}", err),
        };

        let (reply_tx, reply_rx) = tokio::sync::oneshot::channel();

        let req = RequestMessage {
            endname: self.endname.clone(),
            service_method,
            args_type: std::any::type_name::<T>().to_string(),
            args: bytes,
            reply_tx: Some(reply_tx),
        };

        {
            let done = self.done.lock().await;
            if *done {
                return false;
            }
        }

        if self.channel.send(req).await.is_err() {
            return false;
        }
        match reply_rx.await {
            Ok(rep) if rep.success => match bincode::deserialize::<R>(&rep.reply) {
                Ok(decoded) => {
                    *reply = decoded;
                    true
                }
                Err(err) => panic!("ClientEnd::call: decode reply:{:?}", err),
            },
            _ => false,
        }
    }
}
