mod common;
mod server;

#[tokio::main]
async fn main() {
    let _ = server::tcp_networking::init().await;
    server::tasks::launch_server_tasks().await;
}
