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
    let output = Command::new("sw_vers")
        .arg("-productVersion")
        .output()
        .ok()?;

    let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let codename = version_to_codename(&version);

    Some(format!("macOS {codename}"))
}

fn version_to_codename(version: &str) -> String {
    let major = version
        .split('.')
        .next()
        .and_then(|v| v.parse::<u32>().ok())
        .unwrap_or(0);

    match major {
        26 | 25 => "Tahoe",
        24 => "Sequoia",
        23 => "Sonoma",
        22 => "Ventura",
        21 => "Monterey",
        20 => "Big Sur",
        19 => "Catalina",
        18 => "Mojave",
        17 => "High Sierra",
        16 => "Sierra",
        15 => "El Capitan",
        14 => "Yosemite",
        13 => "Mavericks",
        12 => "Mountain Lion",
        11 => "Lion",
        10 => "Snow Leopard",
        _ => return version.to_string(),
    }
    .to_string()
}
