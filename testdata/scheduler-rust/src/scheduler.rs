// scheduler.rs
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct Task {
    name: String,
    priority: u8,
    execute_at: Instant,
}

impl Task {
    pub fn new(name: &str, priority: u8, delay: Duration) -> Self {
        Task {
            name: name.to_string(),
            priority,
            execute_at: Instant::now() + delay,
        }
    }

    pub async fn execute(&self) {
        let now = Instant::now();
        if now < self.execute_at {
            tokio::time::sleep(self.execute_at - now).await;
        }
        println!("Executing task: {}", self.name);
    }
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        other.priority
            .cmp(&self.priority)
            .then_with(|| self.execute_at.cmp(&other.execute_at))
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority && self.execute_at == other.execute_at
    }
}

impl Eq for Task {}

pub struct Scheduler {
    queue: BinaryHeap<Task>,
}

impl Scheduler {
    pub fn new() -> Self {
        Scheduler {
            queue: BinaryHeap::new(),
        }
    }

    pub fn add_task(&mut self, task: Task) {
        self.queue.push(task);
    }

    pub async fn run(&mut self) {
        while let Some(task) = self.queue.pop() {
            task.execute().await;
        }
    }
}

