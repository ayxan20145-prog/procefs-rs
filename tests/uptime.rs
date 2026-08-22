#[test]
fn uptime() {
    let uptime = procefs_rs::uptime().unwrap();

    assert!(uptime >= 0.0);
}
