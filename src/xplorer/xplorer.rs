use std::{net::Ipv4Addr, sync::Arc};

use log::{info, warn};
use tokio::net::UdpSocket;

use super::{
    constant::{ENGINE_MULTICAST_ADDRESS, ENGINE_MULTICAST_PORT},
    message::{ReceiverMessage, SenderMessage},
    receiver::Receiver,
    sender::Sender,
    service::Service,
};

//This is just the handler for the xplorer engine
pub struct Xplorer {
    service: Service,
    sender: Sender,
    receiver: Receiver,
}

impl Xplorer {
    pub fn start(service: Service) -> Xplorer {
        let udp_socket = Self::setup_udp_socket();

        let udp_socket_arched = Arc::new(udp_socket);
        let sender = Sender::start(udp_socket_arched.clone());
        let receiver = Receiver::start(udp_socket_arched.clone());

        Xplorer {
            service: service.clone(),
            sender: sender,
            receiver: receiver,
        }
    }

    async fn handle(service: Service, sender: Sender, receiver: Receiver) {
        //Broadcast to the muticast group that it joined the service
        sender
            .send_message(SenderMessage::MessageSend(bincode::serialize(&service).unwrap()))
            .await;

        receiver
            .send_message(ReceiverMessage::MessageStartReceiving)
            .await;
        let mut  receiver_channel= receiver.subscribe();

        while let Ok(received_packet_data)  = receiver_channel.recv().await  {
            
        }


    }

    fn setup_udp_socket() -> UdpSocket {
        let udp_socket = std::net::UdpSocket::bind(format!(
            "{}:{}",
            ENGINE_MULTICAST_ADDRESS, ENGINE_MULTICAST_PORT
        ))
        .unwrap();

        if let Err(why) = udp_socket.join_multicast_v4(
            &ENGINE_MULTICAST_ADDRESS.parse().unwrap(),
            &Ipv4Addr::UNSPECIFIED,
        ) {
            warn!("Failed to join multicast group: {}", why);
        } else {
            info!("Successfully joined multicast group.");
        }

        udp_socket.set_multicast_loop_v4(true).unwrap();

        UdpSocket::from_std(udp_socket).unwrap()
    }

    pub async fn stop(self) {
        self.sender.stop().await;
        self.receiver.stop().await;
    }
}
