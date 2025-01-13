use std::env;

pub fn dekstop() -> String {
    if cfg!(target_os = "macos") {
        mac()
    } else {
        linux() // default
    }
}

fn linux() -> String {
    env::var("XDG_DESKTOP_SESSION")
    .unwrap_or_else(|_| env::var("XDG_CURRENT_DESKTOP")
    .unwrap_or_else(|_| env::var("DESKTOP_SESSION")
    .unwrap_or("unknown".to_string())))
}

fn mac() -> String {
    "Aqua".to_string() // macOS typically Aqua - check properly in a future release
}