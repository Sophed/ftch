use std::env;

pub fn dekstop() -> String {
    if cfg!(target_os = "macos") {
        // macOS typically uses the Aqua interface
        "Aqua".to_string()
    } else {
        // Check for Linux desktop session variables
        env::var("XDG_DESKTOP_SESSION")
            .unwrap_or_else(|_| env::var("XDG_CURRENT_DESKTOP")
            .unwrap_or_else(|_| env::var("DESKTOP_SESSION")
            .unwrap_or("unknown".to_string())))
    }
}
