use std::fs;
use std::process::Command;

pub fn distro() -> String {
    if cfg!(target_os = "macos") {
        // Use `sw_vers` command on macOS to get the product name
        let output = Command::new("sw_vers")
            .arg("-productName")
            .output()
            .expect("failed to execute process");

        let name = String::from_utf8_lossy(&output.stdout);
        return name.trim().to_string();
    } else {
        // Default to Linux logic
        let contents = fs::read_to_string("/etc/os-release").expect("failed to read distro name");
        let line: Vec<&str> = contents.split("\n").collect();
        let name: Vec<&str> = line.get(0).unwrap().split("=").collect();
        return name.get(1).unwrap().replace("\"", "");
    }
}
