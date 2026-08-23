fn main() {
    let loadavg = procefs_rs::loadavg().unwrap();

    println!("One minute loadavg: {}", loadavg.one);
    println!("Five minute loadavg: {}", loadavg.five);
    println!("Fifteen minute loadavg: {}", loadavg.fifteen);
}
