pub mod distro;
pub mod desktop;
pub mod shell;
pub mod uptime;

const C_RESET: &str = "\x1B[39m";
const C_CYAN: &str = "\x1B[36m";
const ACCENT: &str = C_CYAN;

pub fn line(key: &str, value: String) {
    println!("{ACCENT}{key}{C_RESET}: {value}")
}