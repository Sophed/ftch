use std::fs;
use std::io::{
    Error,
    ErrorKind,
};
use std::process::Command;
use std::time::{
    SystemTime,
    UNIX_EPOCH,
};

pub fn uptime() -> String {
    seconds()
        .map(format_duration)
        .unwrap_or_else(|_| "Unknown".to_string())
}

fn seconds() -> Result<i64, Error> {
    if cfg!(target_os = "macos") {
        mac()
    } else if cfg!(target_os = "windows") {
        windows()
    } else {
        linux()
    }
}

fn linux() -> Result<i64, Error> {
    let contents = fs::read_to_string("/proc/uptime")?;

    contents
        .split_once('.')
        .and_then(|(secs, _)| secs.trim().parse().ok())
        .ok_or_else(|| Error::new(ErrorKind::InvalidData, "invalid uptime format"))
}

fn windows() -> Result<i64, Error> {
    let output = Command::new("powershell")
        .args(&[
            "-NoProfile",
            "-Command",
            "((Get-Date) - (Get-CimInstance Win32_OperatingSystem).LastBootUpTime).TotalSeconds",
        ])
        .output()
        .map_err(|_| Error::new(ErrorKind::Other, "failed to execute powershell"))?;

    let output_str = String::from_utf8_lossy(&output.stdout).trim().to_string();

    // Parse the total seconds as a float and convert to i64
    let seconds: f64 = output_str
        .parse()
        .map_err(|_| Error::new(ErrorKind::InvalidData, "invalid seconds format"))?;

    Ok(seconds as i64)
}

fn mac() -> Result<i64, Error> {
    let output = Command::new("sysctl")
        .arg("-n")
        .arg("kern.boottime")
        .output()
        .map_err(|_| Error::new(ErrorKind::Other, "failed to execute sysctl"))?;

    let output_str = String::from_utf8_lossy(&output.stdout);

    let boot_time = output_str
        .split_once(',')
        .and_then(|(first, _)| first.split_once('='))
        .and_then(|(_, time)| time.trim().parse::<i64>().ok())
        .ok_or_else(|| Error::new(ErrorKind::InvalidData, "invalid boot time format"))?;

    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| Error::new(ErrorKind::Other, "system time error"))?
        .as_secs() as i64;

    Ok(current_time - boot_time)
}

fn format_duration(total_seconds: i64) -> String {
    const MINUTE: i64 = 60;
    const HOUR: i64 = MINUTE * 60;
    const DAY: i64 = HOUR * 24;
    const WEEK: i64 = DAY * 7;
    const YEAR: i64 = DAY * 365;

    const UNITS: &[(i64, &str)] = &[
        (YEAR, "y"),
        (WEEK, "w"),
        (DAY, "d"),
        (HOUR, "h"),
        (MINUTE, "m"),
        (1, "s"),
    ];

    let mut remaining = total_seconds;
    let mut parts = Vec::new();

    for &(unit, suffix) in UNITS {
        let value = remaining / unit;
        remaining %= unit;

        if value > 0 && !(suffix == "s" && total_seconds >= HOUR) {
            parts.push(format!("{value}{suffix}"));
        }
    }

    parts.join(" ")
}
