use std::env;

pub fn desktop() -> String {
    if cfg!(target_os = "macos") {
        mac()
    } else if cfg!(target_os = "windows") {
        windows()
    } else {
        linux()
    }
}

fn linux() -> String {
    [
        "XDG_DESKTOP_SESSION",
        "XDG_CURRENT_DESKTOP",
        "XDG_SESSION_DESKTOP",
        "DESKTOP_SESSION",
    ]
    .iter()
    .find_map(|var| env::var(var).ok())
    .unwrap_or_else(|| "Unknown".to_string())
}

fn windows() -> String {
    "dwm.exe".to_string()
}

fn mac() -> String {
    "Aqua".to_string()
}
