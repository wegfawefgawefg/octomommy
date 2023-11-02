use crate::{
    common::{
        client_to_server::ClientToServerMessageData, server_to_client::ServerToClientMessage,
    },
    server::enque_outbound_messages::send_to_one_client,
};

use super::tcp_networking::INCOMING_MESSAGE_QUEUE;

pub async fn process_message_queue() {
    loop {
        while let Some(message_bundle) = INCOMING_MESSAGE_QUEUE.pop() {
            let connection_id = message_bundle.connection_id;
            match message_bundle.message {
                ClientToServerMessageData::Message { message } => {
                    println!("{} says: {}", connection_id, message);

                    // respond to it
                    let msg = format!("rcvd ur message at {:?}", message_bundle.received_time);
                    let outbound_message = ServerToClientMessage::Message { message: msg };
                    send_to_one_client(connection_id, outbound_message).await;
                }
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
}
