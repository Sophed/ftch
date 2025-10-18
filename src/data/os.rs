use std::fs;
use std::process::Command;

pub fn distro() -> String {
    if cfg!(target_os = "macos") {
        mac().unwrap_or_else(|| "Unknown MacOS".to_string())
    } else if cfg!(target_os = "windows") {
        windows().unwrap_or_else(|| "Unknown Windows".to_string())
    } else {
        linux().unwrap_or_else(|| "Unknown Unix".to_string())
    }
}

fn linux() -> Option<String> {
    let contents = fs::read_to_string("/etc/os-release").ok()?;

    contents
        .lines()
        .find(|line| line.starts_with("PRETTY_NAME="))
        .or_else(|| contents.lines().find(|line| line.starts_with("NAME=")))
        .and_then(|line| line.split_once('='))
        .map(|(_, value)| value.trim_matches('"').to_string())
}

fn windows() -> Option<String> {
    let output = Command::new("powershell")
        .args(&[
            "-NoProfile",
            "-Command",
            "(Get-CimInstance Win32_OperatingSystem).Caption",
        ])
        .output()
        .ok()?;

    let name = String::from_utf8_lossy(&output.stdout)
        .trim()
        .replace("Microsoft ", "")
        .to_string();

    Some(name)
}

fn mac() -> Option<String> {
    let output = Command::new("sw_vers").arg("-productName").output().ok()?;

    Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
}
