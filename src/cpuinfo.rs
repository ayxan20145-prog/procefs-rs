use std::{fs, io};

pub struct Cpu {
    pub vendor: String,
    pub model: String,
    pub logical_cores: usize,
    pub physical_cores: usize,
}

pub fn cpuinfo() -> io::Result<Cpu> {
    let mut vendor = String::new();
    let mut model = String::new();
    let mut logical_cores = 0;
    let mut physical_cores = 0;

    for line in fs::read_to_string("/proc/cpuinfo")?.lines() {
        let Some((name, value)) = line.split_once(':') else {
            continue;
        };

        let name = name.trim();
        let value = value.trim();

        match name {
            "vendor_id" if vendor.is_empty() => vendor = value.to_string(),
            "model name" if model.is_empty() => model = value.to_string(),
            "processor" => logical_cores += 1,
            "cpu cores" if physical_cores == 0 => physical_cores = value.parse().unwrap(),
            _ => {}
        }
    }

    Ok(Cpu {
        vendor,
        model,
        logical_cores,
        physical_cores,
    })
}
