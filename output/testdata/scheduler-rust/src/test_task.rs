use std::time::{Duration, Instant};
use testdata::scheduler_rust::src::scheduler::Task;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_creation_normal_case() {
        let name = String::from("Test Task");
        let priority = 5;
        let execute_at = Instant::now() + Duration::from_secs(60);
        let task = Task {
            name: name.clone(),
            priority,
            execute_at,
        };
        assert_eq!(task.name, name);
        assert_eq!(task.priority, priority);
        assert!(task.execute_at >= Instant::now());
    }

    #[test]
    fn test_task_creation_edge_case_empty_name() {
        let name = String::from("");
        let priority = 5;
        let execute_at = Instant::now() + Duration::from_secs(60);
        let task = Task {
            name: name.clone(),
            priority,
            execute_at,
        };
        assert_eq!(task.name, name);
        assert_eq!(task.priority, priority);
        assert!(task.execute_at >= Instant::now());
    }

    #[test]
    fn test_task_creation_edge_case_max_priority() {
        let name = String::from("High Priority Task");
        let priority = u8::MAX;
        let execute_at = Instant::now() + Duration::from_secs(60);
        let task = Task {
            name: name.clone(),
            priority,
            execute_at,
        };
        assert_eq!(task.name, name);
        assert_eq!(task.priority, priority);
        assert!(task.execute_at >= Instant::now());
    }

    #[test]
    fn test_task_creation_edge_case_min_priority() {
        let name = String::from("Low Priority Task");
        let priority = u8::MIN;
        let execute_at = Instant::now() + Duration::from_secs(60);
        let task = Task {
            name: name.clone(),
            priority,
            execute_at,
        };
        assert_eq!(task.name, name);
        assert_eq!(task.priority, priority);
        assert!(task.execute_at >= Instant::now());
    }

    #[test]
    fn test_task_creation_boundary_condition_past_execution_time() {
        let name = String::from("Past Task");
        let priority = 5;
        let execute_at = Instant::now() - Duration::from_secs(60);
        let task = Task {
            name: name.clone(),
            priority,
            execute_at,
        };
        assert_eq!(task.name, name);
        assert_eq!(task.priority, priority);
        assert!(task.execute_at <= Instant::now());
    }

    #[test]
    fn test_task_creation_boundary_condition_future_execution_time() {
        let name = String::from("Future Task");
        let priority = 5;
        let execute_at = Instant::now() + Duration::from_secs(60 * 60 * 24 * 365);
        let task = Task {
            name: name.clone(),
            priority,
            execute_at,
        };
        assert_eq!(task.name, name);
        assert_eq!(task.priority, priority);
        assert!(task.execute_at >= Instant::now());
    }
}