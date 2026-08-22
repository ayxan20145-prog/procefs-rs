#[test]
fn cpuinfo() {
    let cpu = procefs_rs::cpuinfo().unwrap();

    assert!(!cpu.vendor.is_empty());
    assert!(!cpu.model.is_empty());
    assert!(cpu.cores > 0);
}
