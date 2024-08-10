#[cfg(test)]
mod tests {
    use super::*;
    use tokio::runtime::Runtime;
    use mockall::{mock, predicate::*};
    use std::sync::{Arc, Mutex};

    // Mocking the Task struct and its execute method
    mock! {
        pub Task {}
        impl Task {
            pub async fn execute(&self);
        }
    }

    #[tokio::test]
    async fn test_run_with_tasks() {
        let mut mock_task1 = MockTask::new();
        let mut mock_task2 = MockTask::new();

        mock_task1.expect_execute().returning(|| async {});
        mock_task2.expect_execute().returning(|| async {});

        let queue = Arc::new(Mutex::new(vec![mock_task1, mock_task2]));
        let mut scheduler = Scheduler { queue };

        scheduler.run().await;

        // Assert that the queue is empty after running
        assert!(scheduler.queue.lock().unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_run_with_empty_queue() {
        let queue = Arc::new(Mutex::new(Vec::new()));
        let mut scheduler = Scheduler { queue };

        scheduler.run().await;

        // Assert that the queue is still empty
        assert!(scheduler.queue.lock().unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_run_with_failing_task() {
        let mut mock_task = MockTask::new();

        mock_task.expect_execute().returning(|| async { panic!("Task failed") });

        let queue = Arc::new(Mutex::new(vec![mock_task]));
        let mut scheduler = Scheduler { queue };

        let result = std::panic::AssertUnwindSafe(scheduler.run()).catch_unwind().await;

        // Assert that the function panicked
        assert!(result.is_err());
    }
}

// Scheduler struct definition for context
pub struct Scheduler {
    pub queue: Arc<Mutex<Vec<MockTask>>>,
}

impl Scheduler {
    pub async fn run(&mut self) {
        while let Some(task) = self.queue.lock().unwrap().pop() {
            task.execute().await;
        }
    }
}