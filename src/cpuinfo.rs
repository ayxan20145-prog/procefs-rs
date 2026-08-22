use std::{fs, io};

pub struct Cpu {
    pub vendor: String,
    pub model: String,
    pub cores: usize,
}

pub fn cpuinfo() -> io::Result<Cpu> {
    let mut vendor = String::new();
    let mut model = String::new();
    let mut cores = 0;

    for line in fs::read_to_string("/proc/cpuinfo")?.lines() {
        let Some((name, value)) = line.split_once(':') else {
            continue;
        };

        let name = name.trim();
        let value = value.trim();

        match name {
            "vendor_id" if vendor.is_empty() => vendor = value.to_string(),
            "model name" if model.is_empty() => model = value.to_string(),
            "processor" => cores += 1,
            _ => {}
        }
    }

    Ok(Cpu {
        vendor,
        model,
        cores,
    })
}
