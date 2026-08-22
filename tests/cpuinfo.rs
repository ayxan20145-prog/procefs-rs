#[test]
fn cpuinfo() {
    let cpu = procefs_rs::cpuinfo().unwrap();

    assert!(!cpu.vendor.is_empty());
    assert!(!cpu.model.is_empty());
    assert!(cpu.logical_cores > 0);
    assert!(cpu.physical_cores > 0);
    assert!(!cpu.flags.is_empty());
}
