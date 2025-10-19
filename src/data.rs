pub mod desktop;
pub mod os;
pub mod shell;
pub mod uptime;

const RESET: &str = "\x1B[39m";

pub fn line(key: &str, value: String, seperator: &String, c_primary: &String, c_accent: &String) {
    println!("{c_accent}{key}{c_primary}{seperator}{value}{RESET}")
}
