use std::{fs, io};

pub fn uptime() -> io::Result<f64> {
    let uptime = fs::read_to_string("/proc/uptime")?
        .split_whitespace()
        .next()
        .unwrap()
        .parse::<f64>()
        .unwrap();

    Ok(uptime)
}
