use common::metadata::MessageOrigin;

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

    let _ = tokio::join!(connection_task, task_loops);
}
