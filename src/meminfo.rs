use std::{fs, io};

pub struct Memory {
    pub mem_total: u64,
    pub mem_free: u64,
    pub mem_available: u64,
    pub mem_used: u64,
    pub swap_total: u64,
    pub swap_free: u64,
    pub swap_used: u64,
}

pub fn meminfo() -> io::Result<Memory> {
    let mut mem_total = 0;
    let mut mem_free = 0;
    let mut mem_available = 0;
    let mut swap_total = 0;
    let mut swap_free = 0;

    for line in fs::read_to_string("/proc/meminfo")?.lines() {
        let mut parts = line.split_whitespace();

        let name = parts.next().unwrap();
        let value = parts.next().unwrap().parse::<u64>().unwrap();

        match name {
            "MemTotal:" => mem_total = value,
            "MemFree:" => mem_free = value,
            "MemAvailable:" => mem_available = value,
            "SwapTotal:" => swap_total = value,
            "SwapFree:" => swap_free = value,
            _ => {}
        }
    }

    Ok(Memory {
        mem_total,
        mem_free,
        mem_available,
        mem_used: mem_total - mem_available,
        swap_total,
        swap_free,
        swap_used: swap_total - swap_free,
    })
}
