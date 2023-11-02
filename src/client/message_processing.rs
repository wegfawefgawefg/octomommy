use crate::common::server_to_client::ServerToClientMessage;

use super::tcp_networking::INCOMING_MESSAGE_QUEUE;

pub async fn process_message_queue() {
    loop {
        while let Some(server_to_client_message) = INCOMING_MESSAGE_QUEUE.pop() {
            match server_to_client_message {
                ServerToClientMessage::Message { message } => {
                    println!("Server says: {}", message);
                }
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
}
