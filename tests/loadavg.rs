#[test]
fn loadavg() {
    let loadavg = procefs_rs::loadavg().unwrap();

    assert!(loadavg.one > 0.0);
    assert!(loadavg.five > 0.0);
    assert!(loadavg.fifteen > 0.0);
}
