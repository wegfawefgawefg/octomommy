use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{atomic::AtomicU32, Arc},
};

use crossbeam::queue::ArrayQueue;
use lazy_static::lazy_static;
use tokio::sync::RwLock;

use crate::common::server_to_client::ServerToClientMessage;

pub type ClientMessageQueue = Arc<ArrayQueue<ServerToClientMessage>>;

lazy_static! {
    pub static ref NEXT_CONNECTION_ID: Arc<AtomicU32> = Arc::new(AtomicU32::new(0));
    pub static ref CONNECTION_OUTBOUND_MAILBOXES: RwLock<HashMap<u32, ClientMessageQueue>> =
        RwLock::new(HashMap::new());
}

////////////////////////    CONNECTION BOOKKEEPING    ////////////////////////
pub fn get_next_connection_id() -> u32 {
    NEXT_CONNECTION_ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
}

pub async fn do_bookkeeping_for_new_connection(socket_address: SocketAddr) -> u32 {
    let connection_id = get_next_connection_id();

    // Insert new mailbox into CONNECTION_OUTBOUND_MAILBOXES
    {
        let mut outbound_mailboxes_write = CONNECTION_OUTBOUND_MAILBOXES.write().await;
        let mailbox = Arc::new(ArrayQueue::new(100));
        outbound_mailboxes_write.insert(connection_id, mailbox);
    }

    println!(
        "New Connected {}. Assigned ID: {}",
        socket_address, connection_id
    );
    connection_id
}

///  Removes client allocated bookkeeping resources.
pub async fn do_bookkeeping_for_remove_connection(id: u32) {
    // Remove mailbox from CONNECTION_OUTBOUND_MAILBOXES
    {
        let mut clients_write = CONNECTION_OUTBOUND_MAILBOXES.write().await;
        clients_write.remove(&id);
    }

    println!("Client {} network resources cleaned up.", id);
}
