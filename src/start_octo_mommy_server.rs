mod common;
mod server;

#[tokio::main]
async fn main() {
    let connection_task = tokio::spawn(server::tcp_networking::init());
    let task_loops = tokio::spawn(server::tasks::launch_tasks());

    let _ = tokio::join!(connection_task, task_loops);
}
