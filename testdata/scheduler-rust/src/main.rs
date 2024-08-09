// main.rs
mod scheduler;

use scheduler::{Scheduler, Task};
use std::time::Duration;
use tokio::main;

#[main]
async fn main() {
    let mut scheduler = Scheduler::new();

    // Add tasks to the scheduler.
    scheduler.add_task(Task::new("Task 1", 2, Duration::from_secs(3)));
    scheduler.add_task(Task::new("Task 2", 1, Duration::from_secs(1)));
    scheduler.add_task(Task::new("Task 3", 3, Duration::from_secs(2)));

    // Run the scheduler.
    scheduler.run().await;
}

