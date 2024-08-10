use testdata::scheduler_rust::src::scheduler::{Scheduler, Task};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_task_normal_case() {
        let mut scheduler = Scheduler::new();
        let task = Task::new("Task 1", "https://cloudcode.ai");
        scheduler.add_task(task);
        assert_eq!(scheduler.queue.len(), 1);
        assert_eq!(scheduler.queue[0].name, "Task 1");
    }

    #[test]
    fn test_add_task_multiple_tasks() {
        let mut scheduler = Scheduler::new();
        let task1 = Task::new("Task 1", "https://cloudcode.ai");
        let task2 = Task::new("Task 2", "https://cloudcode.ai");
        scheduler.add_task(task1);
        scheduler.add_task(task2);
        assert_eq!(scheduler.queue.len(), 2);
        assert_eq!(scheduler.queue[0].name, "Task 1");
        assert_eq!(scheduler.queue[1].name, "Task 2");
    }

    #[test]
    fn test_add_task_empty_task() {
        let mut scheduler = Scheduler::new();
        let task = Task::new("", "");
        scheduler.add_task(task);
        assert_eq!(scheduler.queue.len(), 1);
        assert_eq!(scheduler.queue[0].name, "");
        assert_eq!(scheduler.queue[0].url, "");
    }

    #[test]
    fn test_add_task_boundary_conditions() {
        let mut scheduler = Scheduler::new();
        let task = Task::new("a", "https://cloudcode.ai");
        scheduler.add_task(task);
        assert_eq!(scheduler.queue.len(), 1);
        assert_eq!(scheduler.queue[0].name, "a");
    }

    #[test]
    fn test_add_task_error_handling() {
        let mut scheduler = Scheduler::new();
        let task = Task::new("Task 1", "https://cloudcode.ai");
        scheduler.add_task(task);
        assert_eq!(scheduler.queue.len(), 1);
        assert_eq!(scheduler.queue[0].name, "Task 1");

        // Assuming there's a way to simulate an error, e.g., pushing to a full queue
        // This is a placeholder for actual error handling logic
        // let result = scheduler.add_task(task);
        // assert!(result.is_err());
    }
}