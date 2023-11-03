use common::metadata::MessageOrigin;
use tokio::io;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};

mod client;
mod common;

const SERVER_ADDR: &str = "127.0.0.1:8080";

#[tokio::main]
async fn main() {
    let connection_task = tokio::spawn(client::tcp_networking::init(
        SERVER_ADDR,
        MessageOrigin::Cli,
    ));
    let task_loops = tokio::spawn(client::tasks::launch_tasks());

    let message = "uh hi im joining";
    let _send_text = client::tcp_networking::send_text(message).await;

    // User input handling task
    let input_task = tokio::spawn(async {
        let stdin = io::stdin();
        let reader = io::BufReader::new(stdin);
        let mut lines = reader.lines();

        while let Ok(Some(line)) = lines.next_line().await {
            let message = line.trim_end().to_string();
            // Here you would send the message to the server
            // For example, assuming you have a function in your client module
            // that handles sending messages to the server.
            client::tcp_networking::send_text(&message).await
        }
    });

    let _ = tokio::join!(connection_task, task_loops, input_task);
}
