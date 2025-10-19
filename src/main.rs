use std::error::Error;

mod config;
mod data;

fn main() -> Result<(), Box<dyn Error>> {
    let first_run = config::init_config()?;
    let cfg = if first_run {
        config::config::default()
    } else {
        config::read_config()?
    };

    let sep = cfg.display.seperator;
    let pri = cfg.colours.primary;
    let acc = cfg.colours.accent;

    for module_name in cfg.display.modules.iter().map(|n| n.as_str()) {
        match module_name {
            "os" => data::line("OS", data::os::distro(), &sep, &pri, &acc),
            "desktop" => data::line("DE", data::desktop::desktop(), &sep, &pri, &acc),
            "shell" => data::line("SH", data::shell::shell(), &sep, &pri, &acc),
            "uptime" => data::line("UP", data::uptime::uptime(), &sep, &pri, &acc),
            _ => panic!("unknown module '{}'", module_name),
        }
    }

    // print!("┌──────┐    ");
    // print!("│ ┌────┴─┐  ");
    // print!("└─┤  >_  │  ");
    // print!("  └──────┘  ");

    Ok(())
}
