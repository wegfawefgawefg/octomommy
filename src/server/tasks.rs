use super::message_processing::process_message_queue;

pub async fn launch_tasks() {
    let message_queue_task = tokio::spawn(process_message_queue());
    let _ = tokio::join!(message_queue_task);
    // need some form of scheduler here for intermediate tasks
    /* check into
    task scope
    async nursery

    or, just use tokio intervals

    ex:
    async fn run_interval<F: Fn() -> T + Send + 'static, T: Future<Output = ()> + Send>(duration: Duration, f: F) {
        let mut interval = interval(duration);
        loop {
            interval.tick().await;
            let task = f();
            task.await;
        }
    }

    #[tokio::main]
    async fn main() {
        tokio::spawn(run_interval(Duration::from_secs(5), || your_async_task_1()));
        tokio::spawn(run_interval(Duration::from_secs(10), || your_async_task_2()));
        // ...
    }
    */
}
