//! Static build info for Waves Renaissance Bass.

pub const BUILD_VERSION: &str  = "v3.1.13";
pub const BUILD_CHANNEL: &str  = "stable";
pub const BUILD_COMMIT: &str   = "d6fbfdb8d56b";
pub const BUILD_TIMESTAMP: u64 = 1768241261;

pub fn version_tuple() -> (u16, u16, u16) {
    let mut parts = BUILD_VERSION.trim_start_matches('v').split('.');
    let major = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let minor = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let patch = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    (major, minor, patch)
}

pub fn user_agent() -> String {
    format!("{}/{} ({})", "Waves Renaissance Bass", BUILD_VERSION, BUILD_CHANNEL)
}
