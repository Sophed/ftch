use std::fs;

pub fn distro() -> String {
    let contents = fs::read_to_string("/etc/os-release").expect("failed to read distro name");
    let line: Vec<&str> = contents.split("\n").collect();
    let name: Vec<&str> = line.get(0).unwrap().split("=").collect();
    return name.get(1).unwrap().replace("\"", "");
}