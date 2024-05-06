//! # sys_info
//! `sys_info` is a crate that contains functions and structs for getting information about the OS,
//! CPU, RAM, desktop manager, etc...
//!
//! Currently this crate is only available for Linux.

#[cfg(target_os = "linux")]
pub mod linux;

use std::{collections::HashMap, io};

/// Returns the processor name
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub fn processor_name() -> String {
    use raw_cpuid::CpuId;

    match CpuId::new().get_processor_brand_string() {
        Some(name) => name.as_str().to_string(),
        None => String::from("Unknown"),
    }
}

/// Returns the vulnerabilities that the CPU is affected or an [`std::io::Error`].
/// The vulnerabilities are stored in a [`std::collections::HashMap`] where the key is the
/// vulnerability and the value is how is mitigated by the OS.
#[cfg(target_os = "linux")]
pub fn get_cpu_vulnerabilities() -> Result<HashMap<String, String>, io::Error> {
    linux::cpu::get_cpu_vulnerabilities()
}

/// Returns the architecture of the computer or an [`std::io::Error`]
#[cfg(target_os = "linux")]
pub fn arch() -> Result<String, io::Error> {
    linux::kernel::arch()
}

/// Returns the hostname of the machine or an [`std::io::Error`]
#[cfg(target_os = "linux")]
pub fn hostname() -> Result<String, io::Error> {
    linux::network::hostname()
}
