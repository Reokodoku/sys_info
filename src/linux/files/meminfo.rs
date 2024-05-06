use std::{fs::read_to_string, io, path::Path};

#[derive(Default, Clone, Debug)]
pub struct Meminfo {
    mem_total: String,
    swap_total: String,
}

impl Meminfo {
    pub fn parse() -> Result<Self, io::Error> {
        let mut s: Self = Default::default();

        if !Path::new("/proc/meminfo").exists() {
            return Err(io::ErrorKind::NotFound.into());
        }

        for line in read_to_string("/proc/meminfo")?.lines() {
            let splitted: Vec<&str> = line.split(":").collect();

            // Check the key and set the value
            match splitted[0] {
                "MemTotal" => s.mem_total = splitted[1].trim().to_string(),
                "SwapTotal" => s.swap_total = splitted[1].trim().to_string(),
                _ => {}
            }
        }
        Ok(s)
    }

    pub fn mem_total(&self) -> &String {
        &self.mem_total
    }

    pub fn swap_total(&self) -> &String {
        &self.swap_total
    }
}
