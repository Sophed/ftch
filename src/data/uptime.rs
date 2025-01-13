use std::{fs, io::Error, process::Command, time::{SystemTime, UNIX_EPOCH}};

use jiff::SignedDuration;

pub fn uptime() -> String {
    match seconds() {
        Ok(seconds) => format_duration(seconds),
        Err(_) => "unknown".to_string(),
    }
}

fn seconds() -> Result<i64, Error> {
    if cfg!(target_os = "macos") {
        // Use `sysctl` command on macOS to get the system uptime
        let output = Command::new("sysctl")
            .arg("-n")
            .arg("kern.boottime")
            .output()
            .expect("failed to execute process");

        let output_str = String::from_utf8_lossy(&output.stdout);
        
        // Extract the boot time in seconds
        let boot_time_str = output_str.split(',').next().unwrap_or("").split('=').nth(1).unwrap_or("").trim();
        let boot_time = boot_time_str.parse::<i64>().unwrap_or(0);

        // Calculate current time in seconds since UNIX epoch
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        // Calculate uptime
        let uptime_seconds = current_time - boot_time;
        Ok(uptime_seconds)
    } else {
        // Default to Linux logic
        match fs::read_to_string("/proc/uptime") {
            Ok(s) => {
                let parts: Vec<&str> = s.split(".").collect();
                let seconds = parts.get(0).unwrap();
                Ok(seconds.parse::<i64>().unwrap())
            },
            Err(e) => Err(e),
        }
    }
}

fn format_duration(seconds: i64) -> String {
    let years = seconds / (60 * 60 * 24 * 365);
    let weeks = (seconds % (60 * 60 * 24 * 365)) / (60 * 60 * 24 * 7);
    let days = (seconds % (60 * 60 * 24 * 7)) / (60 * 60 * 24);
    let hours = (seconds % (60 * 60 * 24)) / (60 * 60);
    let minutes = (seconds % (60 * 60)) / 60;
    let seconds = seconds % 60;

    let mut parts = vec![];

    if years > 0 {
        parts.push(format!("{}y", years));
    }
    if weeks > 0 {
        parts.push(format!("{}w", weeks));
    }
    if days > 0 {
        parts.push(format!("{}d", days));
    }
    if hours > 0 {
        parts.push(format!("{}h", hours));
    }
    if minutes > 0 {
        parts.push(format!("{}m", minutes));
    }
    parts.push(format!("{}s", seconds)); // Always include seconds

    parts.join(" ")
}
