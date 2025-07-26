use super::*;

#[test]
fn add_task_increments_id() {
    let mut tasks = Vec::new();
    add_task(&"Task 1".to_string(), &mut tasks).unwrap();
    add_task(&"Task 2".to_string(), &mut tasks).unwrap();

    assert_eq!(tasks.len(), 2);
    assert_eq!(tasks[0].id, 1);
    assert_eq!(tasks[1].id, 2);
}

#[test]
fn remove_correct_task() {
    let mut tasks = vec![
        Task {
            id: 1,
            description: "Task 1".to_string(),
            done: false,
        },
        Task {
            id: 2,
            description: "Task 2".to_string(),
            done: false,
        },
    ];

    remove_task(&1, &mut tasks).unwrap();

    assert_eq!(tasks.len(), 1);
    assert_eq!(tasks[0].id, 2);
}

#[test]
fn complete_correct_task() {
    let mut tasks = vec![
        Task {
            id: 1,
            description: "Task 1".to_string(),
            done: false,
        },
        Task {
            id: 2,
            description: "Task 2".to_string(),
            done: false,
        },
    ];

    finish_task(&1, &mut tasks).unwrap();

    assert_eq!(tasks.len(), 2);
    assert_eq!(tasks[0].done, true);
    assert_eq!(tasks[1].done, false);
}
