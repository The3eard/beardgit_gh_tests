use tasklog::store::Store;

#[test]
fn add_then_list_returns_open_task() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("tasks.json");
    let mut store = Store::load_from(&path).unwrap();

    store.add("Write the changelog".into(), Some("docs".into()), None);

    let open = store.list(false);
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

    store.add("Pick up groceries".into(), None, None);
    store.mark_done(1).unwrap();

    assert!(store.list(false).is_empty());
    assert_eq!(store.list(true).len(), 1);
}

#[test]
fn overdue_only_when_open_and_past_due() {
    use chrono::NaiveDate;
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("tasks.json");
    let mut store = Store::load_from(&path).unwrap();

    let past = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();
    let today = NaiveDate::from_ymd_opt(2026, 4, 27).unwrap();

    store.add("Old task".into(), None, Some(past));
    let open = store.list(false);
    assert!(open[0].is_overdue(today));

    store.mark_done(1).unwrap();
    let all = store.list(true);
    assert!(!all[0].is_overdue(today));
}
