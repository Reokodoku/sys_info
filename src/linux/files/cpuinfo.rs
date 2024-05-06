use std::{fs::read_to_string, io, path::Path};

#[derive(Clone, Default, Debug)]
pub struct ProcessorEntry {
    processor: u8,
    cpu_mhz: f64,
    cpu_cores: u8,
}

impl ProcessorEntry {
    pub fn parse(section: String) -> Self {
        let mut s: Self = Default::default();
        for line in section.lines() {
            let splitted: Vec<&str> = line.split(":").collect();

            match splitted[0].trim() {
                "processor" => s.processor = splitted[1].trim().to_string().parse().unwrap(),
                "cpu mhz" => s.cpu_mhz = splitted[1].trim().to_string().parse().unwrap(),
                "cpu cores" => s.cpu_cores = splitted[1].trim().to_string().parse().unwrap(),
                _ => {}
            }
        }

        s
    }

    pub fn processor(&self) -> &u8 {
        &self.processor
    }

    pub fn cpu_mhz(&self) -> &f64 {
        &self.cpu_mhz
    }

    pub fn cpu_cores(&self) -> &u8 {
        &self.cpu_cores
    }
}

#[derive(Clone, Default, Debug)]
pub struct InformationEntry {
    revision: String,
    serial: String,
    model: String,
}

impl InformationEntry {
    pub fn parse(section: String) -> Self {
        let mut s: Self = Default::default();
        for line in section.lines() {
            let splitted: Vec<&str> = line.split(":").collect();

            match splitted[0].trim() {
                "revision" => s.revision = splitted[1].trim().to_string(),
                "serial" => s.serial = splitted[1].trim().to_string(),
                "model" => s.model = splitted[1].trim().to_string(),
                _ => {}
            }
        }

        s
    }

    pub fn revision(&self) -> &String {
        &self.revision
    }

    pub fn serial(&self) -> &String {
        &self.serial
    }

    pub fn model(&self) -> &String {
        &self.model
    }
}

#[derive(Clone, Debug)]
pub enum CpuEntry {
    ProcessorType,
    InformationType,
    Processor(ProcessorEntry),
    Information(InformationEntry),
}

impl CpuEntry {
    pub fn parse(section: String) -> Self {
        let mut entry_type: Option<CpuEntry> = None;

        for line in section.lines() {
            let splitted: Vec<&str> = line.split(":").collect();
            match splitted[0].trim().to_lowercase().as_str() {
                "processor" => entry_type = Some(CpuEntry::ProcessorType),
                _ => entry_type = Some(CpuEntry::InformationType),
            };
            break;
        }

        match entry_type.unwrap() {
            CpuEntry::ProcessorType => CpuEntry::Processor(ProcessorEntry::parse(section)),
            CpuEntry::InformationType => CpuEntry::Information(InformationEntry::parse(section)),
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Cpuinfo {
    entries: Vec<CpuEntry>,
}

impl Cpuinfo {
    pub fn parse() -> Result<Self, io::Error> {
        if !Path::new("/proc/cpuinfo").exists() {
            return Err(io::ErrorKind::NotFound.into());
        }

        let mut entries: Vec<CpuEntry> = vec![];
        for section in read_to_string("/proc/cpuinfo")?
            .split("\n\n")
            .collect::<Vec<&str>>()
        {
            if !section.is_empty() {
                entries.push(CpuEntry::parse(section.to_string()));
            }
        }

        Ok(Self { entries })
    }

    pub fn entries(&self) -> &Vec<CpuEntry> {
        &self.entries
    }

    pub fn cores(&self) -> &u8 {
        for entry in self.entries() {
            match entry {
                CpuEntry::Processor(p) => return &p.cpu_cores,
                CpuEntry::Information(_) => continue,
                _ => unreachable!(),
            }
        }

        &0
    }
}
