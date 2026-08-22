fn main() {
    let cpu = procefs_rs::cpuinfo().unwrap();

    println!("Cpu vendor: {}", cpu.vendor);
    println!("Cpu model: {}", cpu.model);
    println!("Cpu cores: {}", cpu.cores);
}
