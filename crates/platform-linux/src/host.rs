//! Linux [`HostInfo`](platform_api::host::HostInfo) from uname, os-release, and machine-id.

use std::fs;
use std::process::Command;

use platform_api::host::HostInfo;

/// Host facts read from the local Linux install.
pub struct LinuxHostInfo;

impl HostInfo for LinuxHostInfo {
    fn os_name(&self) -> String {
        os_name()
    }

    fn hostname(&self) -> String {
        hostname()
    }

    fn username(&self) -> String {
        username()
    }

    fn host_id(&self) -> String {
        host_id()
    }
}

fn hostname() -> String {
    if let Some(name) = uname_n() {
        return name;
    }
    read_trimmed("/proc/sys/kernel/hostname").unwrap_or_default()
}

fn uname_n() -> Option<String> {
    let output = Command::new("uname").arg("-n").output().ok()?;
    if !output.status.success() {
        return None;
    }
    let name = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if name.is_empty() {
        None
    } else {
        Some(name)
    }
}

fn os_name() -> String {
    if let Ok(contents) = fs::read_to_string("/etc/os-release") {
        if let Some(pretty) = os_release_value(&contents, "PRETTY_NAME") {
            return pretty;
        }
        if let Some(name) = os_release_value(&contents, "NAME") {
            return name;
        }
    }
    String::new()
}

fn os_release_value(contents: &str, key: &str) -> Option<String> {
    for line in contents.lines() {
        let line = line.trim();
        let Some(rest) = line.strip_prefix(key) else {
            continue;
        };
        let Some(rest) = rest.strip_prefix('=') else {
            continue;
        };
        let value = rest.trim().trim_matches('"').to_string();
        if !value.is_empty() {
            return Some(value);
        }
    }
    None
}

fn username() -> String {
    std::env::var("USER")
        .or_else(|_| std::env::var("LOGNAME"))
        .unwrap_or_default()
}

/// Stable machine id for this install.
///
/// Prefers `/etc/machine-id`. When that file is missing (some containers and
/// chroots), reads `/var/lib/dbus/machine-id` instead. Both are written once
/// and remain stable for the life of that install. If neither exists, returns
/// an empty string.
fn host_id() -> String {
    read_trimmed("/etc/machine-id")
        .or_else(|| read_trimmed("/var/lib/dbus/machine-id"))
        .unwrap_or_default()
}

fn read_trimmed(path: &str) -> Option<String> {
    let value = fs::read_to_string(path).ok()?.trim().to_string();
    if value.is_empty() {
        None
    } else {
        Some(value)
    }
}

#[cfg(all(test, target_os = "linux"))]
mod tests {
    use super::*;

    #[test]
    fn hostname_is_non_empty() {
        let host = LinuxHostInfo;
        assert!(!host.hostname().is_empty());
    }
}
