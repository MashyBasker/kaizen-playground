use tokio::time::{self, Instant, Duration};
use testdata::scheduler::scheduler::Scheduler;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::test]
async fn test_execute_normal_case() {
    let execute_at = Instant::now() + Duration::from_secs(1);
    let scheduler = Scheduler { execute_at };
    let scheduler = Arc::new(Mutex::new(scheduler));

    let start = Instant::now();
    scheduler.lock().await.execute().await;
    let elapsed = start.elapsed();

    assert!(elapsed >= Duration::from_secs(1));
}

#[tokio::test]
async fn test_execute_edge_case_now_equals_execute_at() {
    let execute_at = Instant::now();
    let scheduler = Scheduler { execute_at };
    let scheduler = Arc::new(Mutex::new(scheduler));

    let start = Instant::now();
    scheduler.lock().await.execute().await;
    let elapsed = start.elapsed();

    assert!(elapsed < Duration::from_millis(10));
}

#[tokio::test]
async fn test_execute_edge_case_now_greater_than_execute_at() {
    let execute_at = Instant::now() - Duration::from_secs(1);
    let scheduler = Scheduler { execute_at };
    let scheduler = Arc::new(Mutex::new(scheduler));

    let start = Instant::now();
    scheduler.lock().await.execute().await;
    let elapsed = start.elapsed();

    assert!(elapsed < Duration::from_millis(10));
}

#[tokio::test]
async fn test_execute_boundary_condition_just_before_execute_at() {
    let execute_at = Instant::now() + Duration::from_millis(1);
    let scheduler = Scheduler { execute_at };
    let scheduler = Arc::new(Mutex::new(scheduler));

    let start = Instant::now();
    scheduler.lock().await.execute().await;
    let elapsed = start.elapsed();

    assert!(elapsed >= Duration::from_millis(1));
}

#[tokio::test]
async fn test_execute_boundary_condition_just_after_execute_at() {
    let execute_at = Instant::now() - Duration::from_millis(1);
    let scheduler = Scheduler { execute_at };
    let scheduler = Arc::new(Mutex::new(scheduler));

    let start = Instant::now();
    scheduler.lock().await.execute().await;
    let elapsed = start.elapsed();

    assert!(elapsed < Duration::from_millis(10));
}