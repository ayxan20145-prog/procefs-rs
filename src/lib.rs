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

pub struct Memory {
    pub total: u64,
    pub free: u64,
    pub available: u64,
}

pub fn meminfo() -> io::Result<Memory> {
    let mut total = 0;
    let mut free = 0;
    let mut available = 0;

    for line in fs::read_to_string("/proc/meminfo")?.lines() {
        let mut parts = line.split_whitespace();

        let name = parts.next().unwrap();
        let value = parts.next().unwrap().parse::<u64>().unwrap();

        match name {
            "MemTotal:" => total = value,
            "MemFree:" => free = value,
            "MemAvailable:" => available = value,
            _ => {}
        }
    }

    Ok(Memory {
        total,
        free,
        available,
    })
}
