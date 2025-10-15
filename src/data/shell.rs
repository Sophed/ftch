use std::env;

pub fn shell() -> String {
    if cfg!(target_os = "windows") {
        windows_shell()
    } else {
        unix_shell()
    }
}

fn unix_shell() -> String {
    env::var("SHELL")
        .ok()
        .and_then(|s| basename(&s))
        .unwrap_or_else(|| "Unknown".to_string())
}

fn windows_shell() -> String {
    if let Ok(comspec) = env::var("COMSPEC") {
        if let Some(name) = basename(&comspec) {
            return name;
        }
    }

    if env::var("PSModulePath").is_ok() {
        return "PowerShell".to_string();
    }

    "cmd.exe".to_string()
}

fn basename(s: &str) -> Option<String> {
    s.rsplit(['/', '\\']).next().map(|name| name.to_string())
}
