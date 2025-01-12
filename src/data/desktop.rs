use std::env;

pub fn dekstop() -> String {
    env::var("XDG_DESKTOP_SESSION")
        .unwrap_or(env::var("XDG_CURRENT_DESKTOP")
        .unwrap_or(env::var("DESKTOP_SESSION")
        .unwrap_or("unknown".to_string())))
}