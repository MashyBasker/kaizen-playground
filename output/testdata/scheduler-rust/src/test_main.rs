#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use async_std::task;
    use mockall::{predicate::*, Mock};
    
    #[derive(Debug, Clone)]
    struct Task {
        name: String,
        priority: u8,
        duration: Duration,
    }
    
    impl Task {
        fn new(name: &str, priority: u8, duration: Duration) -> Self {
            Task {
                name: name.to_string(),
                priority,
                duration,
            }
        }
    }
    
    #[derive(Default)]
    struct Scheduler {
        tasks: Vec<Task>,
    }
    
    impl Scheduler {
        fn new() -> Self {
            Scheduler { tasks: Vec::new() }
        }
        
        fn add_task(&mut self, task: Task) {
            self.tasks.push(task);
        }
        
        async fn run(&self) {
            for task in &self.tasks {
                task::sleep(task.duration).await;
                println!("Running {}", task.name);
            }
        }
    }
    
    #[async_std::test]
    async fn test_scheduler_with_multiple_tasks() {
        let mut scheduler = Scheduler::new();
        scheduler.add_task(Task::new("Task 1", 2, Duration::from_secs(3)));
        scheduler.add_task(Task::new("Task 2", 1, Duration::from_secs(1)));
        scheduler.add_task(Task::new("Task 3", 3, Duration::from_secs(2)));
        
        scheduler.run().await;
        
        assert_eq!(scheduler.tasks.len(), 3);
    }
    
    #[async_std::test]
    async fn test_scheduler_with_no_tasks() {
        let scheduler = Scheduler::new();
        
        scheduler.run().await;
        
        assert_eq!(scheduler.tasks.len(), 0);
    }
    
    #[async_std::test]
    async fn test_scheduler_with_single_task() {
        let mut scheduler = Scheduler::new();
        scheduler.add_task(Task::new("Task 1", 2, Duration::from_secs(3)));
        
        scheduler.run().await;
        
        assert_eq!(scheduler.tasks.len(), 1);
    }
    
    #[async_std::test]
    async fn test_scheduler_task_order() {
        let mut scheduler = Scheduler::new();
        scheduler.add_task(Task::new("Task 1", 2, Duration::from_secs(3)));
        scheduler.add_task(Task::new("Task 2", 1, Duration::from_secs(1)));
        scheduler.add_task(Task::new("Task 3", 3, Duration::from_secs(2)));
        
        scheduler.run().await;
        
        assert_eq!(scheduler.tasks[0].name, "Task 1");
        assert_eq!(scheduler.tasks[1].name, "Task 2");
        assert_eq!(scheduler.tasks[2].name, "Task 3");
    }
}