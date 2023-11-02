use std::{
    net::SocketAddr,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use crossbeam::queue::ArrayQueue;
use lazy_static::lazy_static;
use tokio::{
    io::{self, AsyncReadExt, AsyncWriteExt},
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpListener, TcpStream,
    },
};

use super::{
    client_bookkeeping::{do_bookkeeping_for_remove_connection, CONNECTION_OUTBOUND_MAILBOXES},
    settings::SERVER_ADDR,
};
use crate::{
    common::client_to_server::{ClientToServerMessage, ClientToServerMessageBundle},
    server::client_bookkeeping::do_bookkeeping_for_new_connection,
};

lazy_static! {
    pub static ref INCOMING_MESSAGE_QUEUE: Arc<ArrayQueue<ClientToServerMessageBundle>> =
        Arc::new(ArrayQueue::new(32));
}

////////////////////////    CLIENT RX/TX TASKS    ////////////////////////

pub async fn init() -> tokio::io::Result<()> {
    println!("Initializing socket...");
    let listener = TcpListener::bind(SERVER_ADDR).await.unwrap();
    println!("Socket Initialized!");
    println!("Awaiting connections...");
    loop {
        let (socket, socket_address) = listener.accept().await?;
        println!("Connection received!");
        tokio::spawn(handle_new_connection(socket, socket_address));
    }
}

pub async fn handle_new_connection(
    socket: TcpStream,
    socket_address: SocketAddr,
) -> tokio::io::Result<()> {
    let connection_id = do_bookkeeping_for_new_connection(socket_address).await;
    //TODO: TOPIC: LOGGING: emit a log message here of a new connection

    println!("Spawning rx/tx tasks...");
    let (read_half, write_half) = socket.into_split();
    let disconnected = Arc::new(AtomicBool::new(false));

    let rx_task_handle = tokio::spawn(continuously_read_inbound_messages(
        read_half,
        socket_address,
        connection_id,
        disconnected.clone(),
    ));
    let tx_task_handle = tokio::spawn(continuously_send_outbound_messages(
        write_half,
        socket_address,
        connection_id,
        disconnected.clone(),
    ));

    // cleanup connection resources when both tasks are done
    let _ = tokio::try_join!(rx_task_handle, tx_task_handle)?;
    do_bookkeeping_for_remove_connection(connection_id).await; //  remove client allocated bookkeeping resources

    Ok(())
}

pub async fn continuously_read_inbound_messages(
    mut socket: OwnedReadHalf,
    socket_address: SocketAddr,
    connection_id: u32,
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

        let result: Result<ClientToServerMessage, _> = bincode::deserialize(&buffer[..nbytes]);
        match result {
            Ok(message) => {
                println!("rcvd a message");
                let received_time = std::time::SystemTime::now();
                let message_bundle = ClientToServerMessageBundle::new(
                    connection_id,
                    socket_address,
                    received_time,
                    message,
                );
                if INCOMING_MESSAGE_QUEUE.push(message_bundle).is_err() {
                    eprintln!(
                        "Inbound message queue full: dropping message from {}",
                        connection_id
                    );
                }
            }
            Err(e) => {
                eprintln!("Error parsing client data: {:?}", e);
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
}

pub async fn continuously_send_outbound_messages(
    mut socket: OwnedWriteHalf,
    _socket_address: SocketAddr,
    id: u32,
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
        {
            let clients_read = CONNECTION_OUTBOUND_MAILBOXES.read().await;
            if let Some(outgoing_messages) = clients_read.get(&id) {
                if let Some(message) = outgoing_messages.pop() {
                    match bincode::serialize(&message) {
                        Ok(binary_message) => {
                            socket.write_all(&binary_message).await?;
                        }
                        Err(e) => {
                            eprintln!("Error serializing message: {:?}", e);
                        }
                    }
                }
            }
        }

        // Some delay, or await on an event to prevent busy-waiting
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
}
