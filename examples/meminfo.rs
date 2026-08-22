fn main() {
    let memory = procefs_rs::meminfo().unwrap();

    println!("Total: {} KB", memory.total);
    println!("Free: {} KB", memory.free);
    println!("Available: {} KB", memory.available);
}
