fn main() {
    let cpu = procefs_rs::cpuinfo().unwrap();

    println!("Cpu vendor: {}", cpu.vendor);
    println!("Cpu model: {}", cpu.model);
    println!("Logical cpu cores: {}", cpu.logical_cores);
    println!("Physical cpu cores: {}", cpu.physical_cores);
    for flag in cpu.flags {
        print!("{} ", flag);
    }
}
