fn main() {
    let uptime = procefs_rs::uptime().unwrap();

    println!("System uptime: {:.2} seconds", uptime);
}
