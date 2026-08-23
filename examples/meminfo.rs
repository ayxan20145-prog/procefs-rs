fn main() {
    let memory = procefs_rs::meminfo().unwrap();

    println!("Memory total: {} KB", memory.mem_total);
    println!("Memory free: {} KB", memory.mem_free);
    println!("Memory available: {} KB", memory.mem_available);
    println!("Memory used: {} KB", memory.mem_used);
    println!("Swap total: {} KB", memory.swap_total);
    println!("Swap free: {} KB", memory.swap_free);
    println!("Swap used: {} KB", memory.swap_used);
}
