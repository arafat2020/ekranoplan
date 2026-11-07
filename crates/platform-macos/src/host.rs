//! macOS [`HostInfo`](platform_api::host::HostInfo) from sysinfo and the platform UUID.

use std::process::Command;

use platform_api::host::HostInfo;
use sysinfo::System;

/// Host facts read from the local macOS install.
pub struct MacOsHostInfo;

impl HostInfo for MacOsHostInfo {
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
    System::host_name().unwrap_or_default()
}

fn os_name() -> String {
    System::long_os_version()
        .or_else(|_| System::name())
        .unwrap_or_default()
}

fn username() -> String {
    std::env::var("USER")
        .or_else(|_| std::env::var("LOGNAME"))
        .unwrap_or_default()
}

/// Stable hardware id for this Mac.
///
/// Reads `IOPlatformUUID` from `IOPlatformExpertDevice` via `ioreg`. That
/// value is set once for the machine (or VM) and stays stable for the life of
/// that install.
fn host_id() -> String {
    let output = match Command::new("ioreg")
        .args(["-rd1", "-c", "IOPlatformExpertDevice"])
        .output()
    {
        Ok(output) if output.status.success() => output,
        _ => return String::new(),
    };

    let text = String::from_utf8_lossy(&output.stdout);
    for line in text.lines() {
        let trimmed = line.trim();
        let Some(rest) = trimmed.strip_prefix("\"IOPlatformUUID\"") else {
            continue;
        };
        let Some(eq) = rest.find('=') else {
            continue;
        };
        let value = rest[eq + 1..].trim().trim_matches('"');
        if !value.is_empty() {
            return value.to_string();
        }
    }
    String::new()
}

#[cfg(all(test, target_os = "macos"))]
mod tests {
    use super::*;

    #[test]
    fn hostname_and_host_id_are_non_empty() {
        let host = MacOsHostInfo;
        assert!(!host.hostname().is_empty());
        assert!(!host.host_id().is_empty());
    }
}
