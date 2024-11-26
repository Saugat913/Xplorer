use log::info;
use std::sync::Arc;
use tokio::{
    net::UdpSocket,
    sync::mpsc::{self, Receiver},
};

use super::{constant::ENGINE_MULTICAST_ADDRESS, message::SenderMessage};

pub struct Sender {
    send_channel: mpsc::Sender<SenderMessage>,
}

impl Sender {
    pub fn start(udp_socket: Arc<UdpSocket>) -> Sender {
        let (tx, mut rx) = mpsc::channel(100);
        tokio::spawn(Self::handle(udp_socket, rx));
        Self { send_channel: tx }
    }

    async fn handle(udp_socket: Arc<UdpSocket>, mut rx: Receiver<SenderMessage>) {
        info!("Starting the sender");
        while let Some(message) = rx.recv().await {
            match message {
                SenderMessage::MessageSend(data) => {
                    udp_socket
                        .send_to(
                            &data,
                            ENGINE_MULTICAST_ADDRESS,
                        )
                        .await
                        .unwrap();
                }
                SenderMessage::MessageStopSender => break,
            }
        }
        info!("Stopping the sender");
    }

    pub async fn send_message(&self, message: SenderMessage) {
        self.send_channel.send(message).await.unwrap();
    }

    pub async fn stop(&self) {
        self.send_message(SenderMessage::MessageStopSender).await;
    }
}
