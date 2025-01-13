use std::fs;
use std::process::Command;

pub fn distro() -> String {
    if cfg!(target_os = "macos") {
        mac()
    } else {
        linux() // default
    }
}

fn linux() -> String {
    let contents = fs::read_to_string("/etc/os-release").expect("failed to read distro name");
    let line: Vec<&str> = contents.split("\n").collect();
    let name: Vec<&str> = line.get(0).unwrap().split("=").collect();
    name.get(1).unwrap().replace("\"", "")
}

fn mac() -> String {
    let output = Command::new("sw_vers")
        .arg("-productName")
        .output()
        .expect("failed to fetch product name");
    
    let name = String::from_utf8_lossy(&output.stdout);
    name.trim().to_string()
}