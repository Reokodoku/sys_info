use std::{fs::read_to_string, io, path::Path};

/// A struct that contains all data from os-release file.
/// Specification: https://www.freedesktop.org/software/systemd/man/latest/os-release.html
#[derive(Default, Clone)]
pub struct OsRelease {
    // General information about the OS
    name: Option<String>,
    id: Option<String>,
    id_like: Option<Vec<String>>,
    pretty_name: Option<String>,
    cpe_name: Option<String>,
    variant: Option<String>,
    variant_id: Option<String>,
    // Information about the version of the OS
    version: Option<String>,
    version_id: Option<String>,
    version_codename: Option<String>,
    build_id: Option<String>,
    image_id: Option<String>,
    image_version: Option<String>,
    // Links and presentation info
    home_url: Option<String>,
    documentation_url: Option<String>,
    support_url: Option<String>,
    bug_report_url: Option<String>,
    privacy_policy_url: Option<String>,
    support_end: Option<String>,
    logo: Option<String>,
    ansi_color: Option<String>,
    vendor_name: Option<String>,
    vendor_url: Option<String>,
    // Metadata and distro defaults
    default_hostname: Option<String>,
    architecture: Option<String>,
    sysext_level: Option<String>,
    confext_level: Option<String>,
    sysext_scope: Option<String>,
    confext_scope: Option<String>,
    portable_prefixes: Option<Vec<String>>,
}

impl OsRelease {
    fn parse_all(path: &str) -> Result<Self, io::Error> {
        let mut s: Self = Default::default();

        for line in read_to_string(path)?.lines() {
            let splitted: Vec<&str> = line.split("=").collect();

            // Check the key and set the value
            match splitted[0] {
                "NAME" => s.name = Some(splitted[1].trim_matches('"').to_string()),
                "ID" => s.id = Some(splitted[1].to_string()),
                "ID_LIKE" => {
                    let mut ids = Vec::new();
                    for id in splitted[1].split(" ") {
                        ids.push(id.to_string());
                    }
                    s.id_like = Some(ids);
                }
                "PRETTY_NAME" => s.pretty_name = Some(splitted[1].trim_matches('"').to_string()),
                "CPE_NAME" => s.cpe_name = Some(splitted[1].trim_matches('"').to_string()),
                "VARIANT" => s.variant = Some(splitted[1].trim_matches('"').to_string()),
                "VARIANT_ID" => s.variant_id = Some(splitted[1].to_string()),

                "VERSION" => s.version = Some(splitted[1].trim_matches('"').to_string()),
                "VERSION_ID" => s.version_id = Some(splitted[1].to_string()),
                "VERSION_CODENAME" => s.version_codename = Some(splitted[1].to_string()),
                "BUILD_ID" => s.build_id = Some(splitted[1].trim_matches('"').to_string()),
                "IMAGE_ID" => s.image_id = Some(splitted[1].to_string()),
                "IMAGE_VERSION" => s.image_version = Some(splitted[1].to_string()),

                "HOME_URL" => s.home_url = Some(splitted[1].trim_matches('"').to_string()),
                "DOCUMENTATION_URL" => {
                    s.documentation_url = Some(splitted[1].trim_matches('"').to_string())
                }
                "SUPPORT_URL" => s.support_url = Some(splitted[1].trim_matches('"').to_string()),
                "BUG_REPORT_URL" => {
                    s.bug_report_url = Some(splitted[1].trim_matches('"').to_string())
                }
                "PRIVACY_POLICY_URL" => {
                    s.privacy_policy_url = Some(splitted[1].trim_matches('"').to_string())
                }
                "SUPPORT_END" => s.support_end = Some(splitted[1].to_string()),
                "LOGO" => s.logo = Some(splitted[1].to_string()),
                "ANSI_COLOR" => s.ansi_color = Some(splitted[1].trim_matches('"').to_string()),
                "VENDOR_NAME" => s.vendor_name = Some(splitted[1].trim_matches('"').to_string()),
                "VENDOR_URL" => s.vendor_url = Some(splitted[1].trim_matches('"').to_string()),

                "DEFAULT_HOSTNAME" => s.default_hostname = Some(splitted[1].to_string()),
                "ARCHITECTURE" => s.architecture = Some(splitted[1].to_string()),
                "SYSEXT_LEVEL" => s.sysext_level = Some(splitted[1].to_string()),
                "CONFEXT_LEVEL" => s.confext_level = Some(splitted[1].to_string()),
                "SYSEXT_SCOPE" => s.sysext_scope = Some(splitted[1].to_string()),
                "CONFEXT_SCOPE" => s.confext_scope = Some(splitted[1].to_string()),
                "PORTABLE_PREFIXES" => {
                    let mut prefixes = Vec::new();
                    for prefix in splitted[1].split(" ") {
                        prefixes.push(prefix.to_string());
                    }
                    s.portable_prefixes = Some(prefixes);
                }

                _ => {}
            }
        }

        Ok(s)
    }

    /// Parses the /etc/os-release file if it exists, otherwise parses /usr/lib/os-release. Can
    /// return [`std::io::Error`]
    pub fn parse() -> Result<Self, io::Error> {
        if Path::new("/etc/os-release").exists() {
            Ok(Self::parse_all("/etc/os-release")?)
        } else if Path::new("/usr/lib/os-release").exists() {
            Ok(Self::parse_all("/usr/lib/os-release")?)
        } else {
            Err(io::ErrorKind::NotFound.into())
        }
    }

    /// Parses the specified file if it exists, otherwise returns [`std::io::Error`]
    pub fn parse_file(path: &str) -> Result<Self, io::Error> {
        if Path::new(path).exists() {
            Ok(Self::parse_all(path)?)
        } else {
            Err(io::ErrorKind::NotFound.into())
        }
    }

    pub fn name(&self) -> &Option<String> {
        &self.name
    }

    pub fn id(&self) -> &Option<String> {
        &self.id
    }

    pub fn id_like(&self) -> &Option<Vec<String>> {
        &self.id_like
    }

    pub fn pretty_name(&self) -> &Option<String> {
        &self.pretty_name
    }

    pub fn cpe_name(&self) -> &Option<String> {
        &self.cpe_name
    }

    pub fn variant(&self) -> &Option<String> {
        &self.variant
    }

    pub fn variant_id(&self) -> &Option<String> {
        &self.variant_id
    }

    pub fn version(&self) -> &Option<String> {
        &self.version
    }

    pub fn version_id(&self) -> &Option<String> {
        &self.version_id
    }

    pub fn version_codename(&self) -> &Option<String> {
        &self.version_codename
    }

    pub fn build_id(&self) -> &Option<String> {
        &self.build_id
    }

    pub fn image_id(&self) -> &Option<String> {
        &self.image_id
    }

    pub fn image_version(&self) -> &Option<String> {
        &self.image_version
    }

    pub fn home_url(&self) -> &Option<String> {
        &self.home_url
    }

    pub fn documentation_url(&self) -> &Option<String> {
        &self.documentation_url
    }

    pub fn support_url(&self) -> &Option<String> {
        &self.support_url
    }

    pub fn bug_report_url(&self) -> &Option<String> {
        &self.bug_report_url
    }

    pub fn privacy_policy_url(&self) -> &Option<String> {
        &self.privacy_policy_url
    }

    pub fn support_end(&self) -> &Option<String> {
        &self.support_end
    }

    pub fn logo(&self) -> &Option<String> {
        &self.logo
    }

    pub fn ansi_color(&self) -> &Option<String> {
        &self.ansi_color
    }

    pub fn vendor_name(&self) -> &Option<String> {
        &self.vendor_name
    }

    pub fn vendor_url(&self) -> &Option<String> {
        &self.vendor_url
    }

    pub fn default_hostname(&self) -> &Option<String> {
        &self.default_hostname
    }

    pub fn architecture(&self) -> &Option<String> {
        &self.architecture
    }

    pub fn sysext_level(&self) -> &Option<String> {
        &self.sysext_level
    }

    pub fn confext_level(&self) -> &Option<String> {
        &self.confext_level
    }

    pub fn sysext_scope(&self) -> &Option<String> {
        &self.sysext_scope
    }

    pub fn confext_scope(&self) -> &Option<String> {
        &self.confext_scope
    }

    pub fn portable_prefixes(&self) -> &Option<Vec<String>> {
        &self.portable_prefixes
    }

    /// Returns pretty_name if it isn't None, otherwise returns name (or None if name isn't
    /// Some(...))
    pub fn get_name(&self) -> &Option<String> {
        if !self.pretty_name.is_none() {
            &self.pretty_name
        } else {
            &self.name
        }
    }

    pub fn get_logo_svg(&self) -> Option<String> {
        if self.logo.is_some() {
            let path = format!("/usr/share/pixmaps/{}.svg", self.logo.as_ref().unwrap());
            if Path::new(&path).exists() {
                return Some(read_to_string(&path).unwrap_or_default());
            }
        }
        None
    }
}
