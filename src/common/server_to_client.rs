use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ServerToClientMessage {
    Connect,
    Disconnect,
    Message { message: String },
}
