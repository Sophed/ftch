use std::fs;
use std::process::Command;

pub fn distro() -> String {
    if cfg!(target_os = "macos") {
        mac()
    } else if cfg!(target_os = "windows") {
        windows()
    } else {
        linux() // default
    }
}

fn linux() -> String {
    let contents = fs::read_to_string("/etc/os-release").expect("failed to read distro name");
    let lines: Vec<&str> = contents.split("\n").collect();
    for line in &lines {
        if line.starts_with("PRETTY_NAME") { // priority
            let name: Vec<&str> = line.split("=").collect();
            return name.get(1).unwrap().replace("\"", "");
        }
    }
    for line in lines {
        if line.starts_with("NAME") { // fallback
            let name: Vec<&str> = line.split("=").collect();
            return name.get(1).unwrap().replace("\"", "");
        }
    }
    "unknown".to_string() // terrible distro config
}

fn windows() -> String {
    let cmd = Command::new("systeminfo")
        .output()
        .expect("failed to fetch system info");
    let out = String::from_utf8_lossy(&cmd.stdout).trim().to_string();
    let line: String = out.split("\n").filter(|l| l.contains("OS Name:")).collect();
    line.replace("OS Name:", "").replace("Microsoft", "").trim().to_string()
}

fn mac() -> String {
    let output = Command::new("sw_vers")
        .arg("-productName")
        .output()
        .expect("failed to fetch product name");

    let name = String::from_utf8_lossy(&output.stdout);
    name.trim().to_string()
}
