use std::time::SystemTime;

use serde::{Deserialize, Serialize};

use super::metadata::MessageOrigin;

#[derive(Debug, Clone)]
pub struct ClientToServerMessageBundle {
    // metadata
    pub connection_id: u32,
    pub socket_address: std::net::SocketAddr,
    pub send_time: SystemTime,
    pub received_time: SystemTime,
    pub origin: MessageOrigin,

    // actual message
    pub message: ClientToServerMessageData,
}

impl ClientToServerMessageBundle {
    pub fn new(
        connection_id: u32,
        socket_address: std::net::SocketAddr,
        received_time: SystemTime,
        message: ClientToServerMessage,
    ) -> Self {
        Self {
            connection_id,
            socket_address,
            send_time: message.send_time,
            received_time,
            origin: message.origin,
            message: message.data,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientToServerMessage {
    // metadata
    pub send_time: SystemTime,
    pub origin: MessageOrigin,

    // message
    pub data: ClientToServerMessageData,
}

impl ClientToServerMessage {
    pub fn new(origin: MessageOrigin, data: ClientToServerMessageData) -> Self {
        Self {
            send_time: SystemTime::now(),
            origin,
            data,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ClientToServerMessageData {
    Message { message: String },
}
