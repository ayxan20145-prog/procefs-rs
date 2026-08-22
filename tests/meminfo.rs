#[test]
fn meminfo() {
    let memory = procefs_rs::meminfo().unwrap();

    assert!(memory.total > 0);
    assert!(memory.free > 0);
    assert!(memory.available > 0);
}
