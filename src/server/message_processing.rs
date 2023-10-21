use std::{collections::HashMap, time::Instant};

use crate::{
    common::{
        client_to_server::{ClientToServerMessage, ClientToServerMessageBundle},
        server_to_client::ServerToClientMessage,
    },
    server::enque_outbound_messages::{broadcast_to_all_except, send_to_one_client},
};

use super::tcp_networking::INCOMING_MESSAGE_QUEUE;

pub const FRAMES_PER_SECOND: u32 = 60;
const TIMESTEP: f32 = 1.0 / FRAMES_PER_SECOND as f32;

pub const DEBUG_PRINT_PROCESSED_MESSAGES: bool = false;

pub async fn process_message_queue() {
    // prune_latest_only_messages().await;

    while let Some(message_bundle) = INCOMING_MESSAGE_QUEUE.pop() {
        let client_id = message_bundle.client_id;
        match message_bundle.message {
            ClientToServerMessage::Connect => {
                println!("Client {} connected", client_id);

                // send welcome
                let outbound_message = ServerToClientMessage::Welcome {
                    server_message: "welcome to the server".to_string(),
                };
                send_to_one_client(client_id, outbound_message).await;

                // announce the join
                let outbound_message = ServerToClientMessage::ClientJoined { id: client_id };
                broadcast_to_all_except(client_id, outbound_message).await;
            }
            ClientToServerMessage::Disconnect => {
                println!("Client {} disconnected", client_id);

                // announce the leave
                let outbound_message = ServerToClientMessage::ClientLeft { id: client_id };
                broadcast_to_all_except(client_id, outbound_message).await;
            }
            ClientToServerMessage::Message { message } => {
                println!("{} says: {}", client_id, message);

                // broadcast the message
                let outbound_message = ServerToClientMessage::Message {
                    from: client_id,
                    message,
                };
                broadcast_to_all_except(client_id, outbound_message).await;
            }
        }
    }
}
