use super::service::Service;

pub enum SenderMessage {
    MessageSend(Vec<u8>),
    MessageStopSender,
}

#[derive(PartialEq, Debug)]
pub enum ReceiverMessage {
    MessageStartReceiving,
    MessageStopReceiving,
    MessageStopReceiver,
}
