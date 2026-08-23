#[test]
fn meminfo() {
    let memory = procefs_rs::meminfo().unwrap();

    assert!(memory.mem_total > 0);
    assert!(memory.mem_free > 0);
    assert!(memory.mem_available > 0);
    assert!(memory.mem_used > 0);
    assert!(memory.swap_free <= memory.swap_total);
    assert!(memory.swap_used <= memory.swap_total);
    assert_eq!(memory.swap_used + memory.swap_free, memory.swap_total);
}
