use std::collections::VecDeque;
use std::error::Error;

mod ascii;
mod config;
mod data;

const RESET: &str = "\x1B[39m";

fn main() -> Result<(), Box<dyn Error>> {
    let first_run = config::init_config()?;
    let cfg = if first_run {
        config::config::default()
    } else {
        config::read_config()?
    };

    let mut modules: VecDeque<&str> = cfg.display.modules.iter().map(|n| n.as_str()).collect();

    let ascii_art = match cfg.display.ascii.as_str() {
        "stack" => ascii::STACK,
        s => return Err(format!("unknown ascii art '{}'", s).into()),
    };
    let max_length = ascii_art
        .lines()
        .max_by_key(|l| l.chars().count())
        .map(|l| l.chars().count())
        .unwrap_or(0);

    for line in ascii_art.lines() {
        let module_name = modules.pop_front().unwrap_or("");
        
        if module_name.is_empty() {
            println!("{line:max_length$}");
            continue;
        }
        
        let (key, value) = match module_name {
            "os" => ("OS", data::os::distro()),
            "desktop" => ("DE", data::desktop::desktop()),
            "shell" => ("SH", data::shell::shell()),
            "uptime" => ("UP", data::uptime::uptime()),
            _ => return Err(format!("unknown module '{}'", module_name).into()),
        };
        
        println!(
            "{line:max_length$}  {accent}{key}{primary}{seperator}{value}{RESET}",
            accent = cfg.colours.accent,
            primary = cfg.colours.primary,
            seperator = cfg.display.seperator,
        );
    }

    Ok(())
}