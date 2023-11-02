use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use crossbeam::queue::ArrayQueue;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use lazy_static::lazy_static;

use crate::common::client_to_server::{ClientToServerMessage, ClientToServerMessageData};
use crate::common::metadata::MessageOrigin;
use crate::common::server_to_client::ServerToClientMessage;

lazy_static! {
    pub static ref INCOMING_MESSAGE_QUEUE: Arc<ArrayQueue<ServerToClientMessage>> =
        Arc::new(ArrayQueue::new(32));
    pub static ref OUTBOUND_MESSAGE_QUEUE: Arc<ArrayQueue<ClientToServerMessageData>> =
        Arc::new(ArrayQueue::new(32));
}

////////////////////////    CLIENT RX/TX TASKS    ////////////////////////

pub async fn init(server_address: &str, origin: MessageOrigin) -> tokio::io::Result<()> {
    let stream = TcpStream::connect(server_address).await?;
    let (read_half, write_half) = stream.into_split();
    let disconnected = Arc::new(AtomicBool::new(false));

    tokio::spawn(receive_incoming_messages(read_half, disconnected.clone()));
    tokio::spawn(transmit_outbound_messages(
        write_half,
        origin,
        disconnected.clone(),
    ));
    Ok(())
}

pub async fn receive_incoming_messages(
    mut socket: tokio::net::tcp::OwnedReadHalf,
    disconnected: Arc<AtomicBool>,
) -> io::Result<()> {
    println!("Listening for incoming messages...");
    let mut buffer = [0; 1024];
    loop {
        let nbytes = socket.read(&mut buffer).await?;

        // check for disconnect
        if nbytes == 0 {
            //TODO: TOPIC: LOGGING: emit a log message here of disconnect
            disconnected.store(true, Ordering::SeqCst);
            return Ok(());
        }

        let result: Result<ServerToClientMessage, _> = bincode::deserialize(&buffer[..nbytes]);
        match result {
            Ok(message) => {
                if INCOMING_MESSAGE_QUEUE.push(message).is_err() {
                    eprintln!("Inbound message queue full: dropping message from server");
                }
            }
            Err(e) => {
                eprintln!("Error parsing client data: {:?}", e);
            }
        }

        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
}

pub async fn transmit_outbound_messages(
    mut socket: tokio::net::tcp::OwnedWriteHalf,
    origin: MessageOrigin,
    disconnected: Arc<AtomicBool>,
) -> io::Result<()> {
    loop {
        // check for disconnect
        {
            if disconnected.load(Ordering::SeqCst) {
                return Ok(());
            }
        }

        // transmit any outbound messages
        if let Some(message_data) = OUTBOUND_MESSAGE_QUEUE.pop() {
            let message = ClientToServerMessage::new(origin.clone(), message_data);
            match bincode::serialize(&message) {
                Ok(binary_message) => {
                    socket.write_all(&binary_message).await?;
                    println!("Sent message: {:?}", message);
                }
                Err(e) => {
                    eprintln!("Error serializing message: {:?}", e);
                }
            }
        }

        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
}

////////////////////////    SEND UTILITY    ////////////////////////
pub async fn send_text(message: &str) {
    println!("Sending message: {}", message);
    if OUTBOUND_MESSAGE_QUEUE
        .push(ClientToServerMessageData::Message {
            message: message.to_string(),
        })
        .is_err()
    {
        eprintln!("Outbound message queue full... :( something is wrong");
    };
}
