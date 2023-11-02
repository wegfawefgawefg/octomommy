use super::message_processing::process_message_queue;

pub async fn launch_tasks() {
    let process_message_queue_task = tokio::spawn(process_message_queue());
    let _ = tokio::join!(process_message_queue_task);
}
