use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ServerToClientMessage {
    Message { message: String },
}
