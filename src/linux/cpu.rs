use std::{collections::HashMap, fs, io};

/// Returns the maximum and the minimum frequency of the specified CPU or an [`std::io::Error`].
/// The frequencies are expressed in KHz.
/// Documentation: https://docs.kernel.org/admin-guide/pm/cpufreq.html
pub fn get_min_max_cpu_freq(n: u8) -> Result<(u64, u64), io::Error> {
    Ok((
        fs::read_to_string(format!(
            "/sys/devices/system/cpu/cpu{}/cpufreq/cpuinfo_min_freq",
            n
        ))?
        .trim()
        .parse()
        .unwrap(),
        fs::read_to_string(format!(
            "/sys/devices/system/cpu/cpu{}/cpufreq/cpuinfo_max_freq",
            n
        ))?
        .trim()
        .parse()
        .unwrap(),
    ))
}

/// Returns the vulnerabilities that the CPU is affected or an [`std::io::Error`].
/// The vulnerabilities are stored in a [`std::collections::HashMap`], where the key is the
/// vulnerability and the value is how it is mitigated by the Linux kernel.
pub fn get_cpu_vulnerabilities() -> Result<HashMap<String, String>, io::Error> {
    let mut vulns: HashMap<String, String> = HashMap::new();

    for vuln in fs::read_dir("/sys/devices/system/cpu/vulnerabilities/")? {
        let vuln = vuln?;
        let vuln_content = fs::read_to_string(vuln.path())?;

        if vuln_content != "Not affected\n" {
            vulns.insert(
                vuln.file_name().into_string().unwrap(),
                vuln_content
                    .strip_prefix("Mitigation: ")
                    .unwrap()
                    .trim()
                    .to_string(),
            );
        }
    }

    Ok(vulns)
}
