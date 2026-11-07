//! Fixed host facts for tests. Does not call the operating system.

use crate::host::HostInfo;

/// Returns the same four strings every time.
pub struct FakeHostInfo;

impl HostInfo for FakeHostInfo {
    fn os_name(&self) -> String {
        "FakeOS".to_string()
    }

    fn hostname(&self) -> String {
        "fake-host".to_string()
    }

    fn username(&self) -> String {
        "fake-user".to_string()
    }

    fn host_id(&self) -> String {
        "fake-host-id".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fake_returns_fixed_strings() {
        let host = FakeHostInfo;
        assert_eq!(host.os_name(), "FakeOS");
        assert_eq!(host.hostname(), "fake-host");
        assert_eq!(host.username(), "fake-user");
        assert_eq!(host.host_id(), "fake-host-id");
    }
}
