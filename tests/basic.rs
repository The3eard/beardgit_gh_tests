use tasklog::store::Store;

#[test]
fn add_then_list_returns_open_task() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("tasks.json");
    let mut store = Store::load_from(&path).unwrap();

    store
        .add(
            "Write the changelog".into(),
            Some("docs".into()),
            None,
            None,
        )
        .unwrap();

    let open = store.list(false, None);
    assert_eq!(open.len(), 1);
    assert_eq!(open[0].title, "Write the changelog");
    assert_eq!(open[0].tag.as_deref(), Some("docs"));
    assert!(!open[0].is_done());
}

#[test]
fn done_then_default_list_hides_completed() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("tasks.json");
    let mut store = Store::load_from(&path).unwrap();

    store
        .add("Pick up groceries".into(), None, None, None)
        .unwrap();
    store.mark_done(1).unwrap();

    assert!(store.list(false, None).is_empty());
    assert_eq!(store.list(true, None).len(), 1);
}

#[test]
fn overdue_only_when_open_and_past_due() {
    use chrono::NaiveDate;
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("tasks.json");
    let mut store = Store::load_from(&path).unwrap();

    let past = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();
    let today = NaiveDate::from_ymd_opt(2026, 4, 27).unwrap();

    store
        .add("Old task".into(), None, Some(past), None)
        .unwrap();
    let open = store.list(false, None);
    assert!(open[0].is_overdue(today));

    store.mark_done(1).unwrap();
    let all = store.list(true, None);
    assert!(!all[0].is_overdue(today));
}

#[test]
fn done_on_missing_id_returns_error_not_panic() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("tasks.json");
    let mut store = Store::load_from(&path).unwrap();

    let err = store.mark_done(42).unwrap_err();
    assert!(err.to_string().contains("no task with id 42"));
}

#[test]
fn rm_on_missing_id_returns_error_not_silent_noop() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("tasks.json");
    let mut store = Store::load_from(&path).unwrap();

    let err = store.remove(99).unwrap_err();
    assert!(err.to_string().contains("no task with id 99"));
}

#[test]
fn search_is_case_insensitive_and_matches_tag() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("tasks.json");
    let mut store = Store::load_from(&path).unwrap();

    store
        .add("Triage parser bug".into(), Some("bug".into()), None, None)
        .unwrap();
    store
        .add("Refactor CLI".into(), Some("chore".into()), None, None)
        .unwrap();

    assert_eq!(store.search("PARSER").len(), 1);
    assert_eq!(store.search("bug").len(), 1);
    assert_eq!(store.search("nonexistent").len(), 0);
}

#[test]
fn marking_recurring_done_spawns_next_occurrence() {
    use chrono::NaiveDate;
    use tasklog::recurrence::Recurrence;

    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("tasks.json");
    let mut store = Store::load_from(&path).unwrap();

    let due = NaiveDate::from_ymd_opt(2026, 5, 1).unwrap();
    store
        .add(
            "Pay rent".into(),
            Some("home".into()),
            Some(due),
            Some(Recurrence::Monthly),
        )
        .unwrap();

    let outcome = store.mark_done(1).unwrap();
    assert_eq!(outcome.completed.id, 1);
    let next_id = outcome.rolled_id.expect("monthly recurrence rolls over");

    let all = store.list(true, None);
    let next = all.iter().find(|t| t.id == next_id).unwrap();
    assert_eq!(next.title, "Pay rent");
    assert_eq!(next.due, NaiveDate::from_ymd_opt(2026, 6, 1));
    assert!(!next.is_done());
}
