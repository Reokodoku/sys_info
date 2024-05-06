use std::{fs, io};

/// Returns the hostname of the machine or an [`std::io::Error`]
pub fn hostname() -> Result<String, io::Error> {
    Ok(fs::read_to_string("/proc/sys/kernel/hostname")?
        .trim_end_matches('\n')
        .to_string())
}
