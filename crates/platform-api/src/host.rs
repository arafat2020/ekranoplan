//! Host identity reported by an operating system.

/// Facts about the machine the agent runs on.
pub trait HostInfo {
    fn os_name(&self) -> String;
    fn hostname(&self) -> String;
    fn username(&self) -> String;
    fn host_id(&self) -> String;
}
