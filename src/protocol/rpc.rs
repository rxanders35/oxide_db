use bincode::{deserialize, serialize};
use std::any::Any;
use tokio::sync::mpsc::{channel, Receiver, Sender};

pub struct RequestMessage {
    endname: Box<dyn Any>,
    svc_method: String,
    args_type: String,
    args: Vec<u8>,
    reply_channel: Sender<ReplyMessage>,
}

pub struct ReplyMessage {
    success: bool,
    reply: Vec<u8>,
}

struct ClientEnd {
    endname: Box<dyn Any>,
    ch: Sender<RequestMessage>,
    done: Receiver<()>,
}

impl ClientEnd {
    pub fn call(&self, svc_method: &str, args: Box<dyn Any>, reply: Box<dyn Any>) -> bool {
        todo!()
    }
}
