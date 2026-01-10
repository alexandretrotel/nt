#[cfg(target_os = "macos")]
pub fn default_iface() -> String {
    use crate::platform::macos::get_active_interface;

    get_active_interface().unwrap_or_else(|| "en0".to_string())
}

#[cfg(not(target_os = "macos"))]
pub fn default_iface() -> String {
    "eth0".to_string()
}
