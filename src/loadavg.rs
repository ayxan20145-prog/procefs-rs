use std::{fs, io};

pub struct LoadAvg {
    pub one: f64,
    pub five: f64,
    pub fifteen: f64,
}

pub fn loadavg() -> io::Result<LoadAvg> {
    let content = fs::read_to_string("/proc/loadavg")?;
    let mut parts = content.split_whitespace();

    let one = parts.next().unwrap().parse::<f64>().unwrap();
    let five = parts.next().unwrap().parse::<f64>().unwrap();
    let fifteen = parts.next().unwrap().parse::<f64>().unwrap();

    Ok(LoadAvg { one, five, fifteen })
}
