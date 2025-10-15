use std::env;

pub fn dekstop() -> String {
    if cfg!(target_os = "macos") {
        mac()
    } else if cfg!(target_os = "windows") {
        windows()
    } else {
        linux() // default
    }
}

fn linux() -> String {
    env::var("XDG_DESKTOP_SESSION").unwrap_or_else(|_| {
        env::var("XDG_CURRENT_DESKTOP").unwrap_or_else(|_| {
            env::var("XDG_SESSION_DESKTOP")
                .unwrap_or_else(|_| env::var("DESKTOP_SESSION").unwrap_or("unknown".to_string()))
        })
    })
}

fn windows() -> String {
    "dwm.exe".to_string() // possible later detect is GlazeWM or komorebi is running and show those
}

fn mac() -> String {
    "Aqua".to_string() // macOS typically Aqua - check properly in a future release
}
