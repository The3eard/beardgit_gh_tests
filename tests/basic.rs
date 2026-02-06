use tasklog::store::Store;

#[test]
fn add_then_list_returns_open_task() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("tasks.json");
    let mut store = Store::load_from(&path).unwrap();

    store.add("Write the changelog".into());

    let open = store.list(false);
    assert_eq!(open.len(), 1);
    assert_eq!(open[0].title, "Write the changelog");
    assert!(!open[0].done);
}

#[test]
fn done_then_default_list_hides_completed() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("tasks.json");
    let mut store = Store::load_from(&path).unwrap();

    store.add("Pick up groceries".into());
    store.mark_done(1).unwrap();

    assert!(store.list(false).is_empty());
    assert_eq!(store.list(true).len(), 1);
}
