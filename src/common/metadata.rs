use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageOrigin {
    Discord,
    Telegram,
    Cli,
}
