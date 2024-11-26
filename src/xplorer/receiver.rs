use std::sync::Arc;

use log::info;
use tokio::{
    net::UdpSocket,
    sync::{
        broadcast,
        mpsc::{self, Sender},
    },
};

use super::{message::ReceiverMessage, service::Service};

pub struct Receiver {
    send_channel: Sender<ReceiverMessage>,
    receiving_channel: broadcast::Sender<Vec<u8>>,
}

impl Receiver {
    pub fn start(udp_socket: Arc<UdpSocket>) -> Receiver {
        let (tx, rx) = mpsc::channel(100);
        let (tx1, _) = broadcast::channel(100);
        tokio::spawn(Self::handle(udp_socket, rx, tx1.clone()));
        Self {
            send_channel: tx,
            receiving_channel: tx1,
        }
    }

    async fn handle(
        udp_socket: Arc<UdpSocket>,
        mut rx: tokio::sync::mpsc::Receiver<ReceiverMessage>,
        mut tx: tokio::sync::broadcast::Sender<Vec<u8>>,
    ) {
        let current_state = ReceiverMessage::MessageStopReceiving;
        let mut receiving_handle = None;
        info!("Started the receiver");
        while let Some(message) = rx.recv().await {
            //Run only when state change
            if current_state != message {
                match message {
                    ReceiverMessage::MessageStartReceiving => {
                        let udp_socket_cloned = udp_socket.clone();
                        let service_found_sender = tx.clone();
                        receiving_handle = Some(tokio::spawn(async move {
                            let mut buffer: Vec<u8> = vec![0; 1024];
                            while let Ok(data_size) = udp_socket_cloned.recv(&mut buffer).await {
                                service_found_sender
                                    .send(buffer[..data_size].to_vec())
                                    .unwrap();
                            }
                        }));
                    }
                    ReceiverMessage::MessageStopReceiving => {
                        receiving_handle = match &receiving_handle {
                            Some(handle) => {
                                handle.abort();
                                info!("Stopping the receiving");
                                None
                            }
                            None => None,
                        }
                    }
                    ReceiverMessage::MessageStopReceiver => {
                        match &receiving_handle {
                            Some(handle) => {
                                handle.abort();
                            }
                            None => {}
                        };
                        break;
                    }
                }
            }
        }
        info!("Stopping the receiver");
    }
    pub async fn send_message(&self, message: ReceiverMessage) {
        self.send_channel.send(message).await.unwrap();
    }
    pub async fn stop(self) {
        self.send_message(ReceiverMessage::MessageStopReceiver)
            .await;
    }
    pub fn subscribe(&self) -> broadcast::Receiver<Vec<u8>> {
        self.receiving_channel.subscribe()
    }
}
