use std::{fs, io};

/// Returns the current kernel version or an [`std::io::Error`]
pub fn kernel_version() -> Result<String, io::Error> {
    fs::read_to_string("/proc/sys/kernel/osrelease")
}

/// Returns the architecture of the computer (e.g. x86, arm, riscv) or an [`std::io::Error`]
pub fn arch() -> Result<String, io::Error> {
    fs::read_to_string("/proc/sys/kernel/arch")
}
