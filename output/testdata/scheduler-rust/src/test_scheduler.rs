#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BinaryHeap;

    #[derive(Eq, PartialEq, Ord, PartialOrd)]
    struct Task {
        priority: u32,
        description: String,
    }

    struct Scheduler {
        queue: BinaryHeap<Task>,
    }

    impl Scheduler {
        fn new() -> Self {
            Scheduler {
                queue: BinaryHeap::new(),
            }
        }

        fn add_task(&mut self, task: Task) {
            self.queue.push(task);
        }

        fn pop_task(&mut self) -> Option<Task> {
            self.queue.pop()
        }

        fn peek_task(&self) -> Option<&Task> {
            self.queue.peek()
        }
    }

    #[test]
    fn test_add_task() {
        let mut scheduler = Scheduler::new();
        let task = Task { priority: 1, description: String::from("Test Task") };
        scheduler.add_task(task);
        assert_eq!(scheduler.queue.len(), 1);
    }

    #[test]
    fn test_pop_task() {
        let mut scheduler = Scheduler::new();
        let task1 = Task { priority: 1, description: String::from("Task 1") };
        let task2 = Task { priority: 2, description: String::from("Task 2") };
        scheduler.add_task(task1);
        scheduler.add_task(task2);
        let popped_task = scheduler.pop_task().unwrap();
        assert_eq!(popped_task.description, "Task 2");
        assert_eq!(scheduler.queue.len(), 1);
    }

    #[test]
    fn test_peek_task() {
        let mut scheduler = Scheduler::new();
        let task1 = Task { priority: 1, description: String::from("Task 1") };
        let task2 = Task { priority: 2, description: String::from("Task 2") };
        scheduler.add_task(task1);
        scheduler.add_task(task2);
        let peeked_task = scheduler.peek_task().unwrap();
        assert_eq!(peeked_task.description, "Task 2");
    }

    #[test]
    fn test_empty_scheduler() {
        let scheduler = Scheduler::new();
        assert!(scheduler.peek_task().is_none());
        assert!(scheduler.pop_task().is_none());
    }

    #[test]
    fn test_boundary_conditions() {
        let mut scheduler = Scheduler::new();
        for i in 0..1000 {
            scheduler.add_task(Task { priority: i, description: format!("Task{}", i) });
        }
        assert_eq!(scheduler.queue.len(), 1000);
        for i in (0..1000).rev() {
            let task = scheduler.pop_task().unwrap();
            assert_eq!(task.priority, i);
        }
        assert!(scheduler.queue.is_empty());
    }
}