use std::{fs, io::Error};

use jiff::SignedDuration;

pub fn uptime() -> String {
    match seconds() {
        Ok(seconds_str) => {
            let seconds = seconds_str.parse::<i64>().unwrap();
            let dur = SignedDuration::from_secs(seconds);
            format!("{dur:#}")
        },
        Err(_) => "unknown".to_string(),
    }
}

fn seconds() -> Result<String, Error> {
    match fs::read_to_string("/proc/uptime") {
        Ok(s) => {
            let parts: Vec<&str> = s.split(".").collect();
            let seconds = parts.get(0).unwrap();
            Ok(seconds.to_string())
        },
        Err(e) => Err(e),
    }
}