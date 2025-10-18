use std::error::Error;

mod config;
mod data;

fn main() -> Result<(), Box<dyn Error>> {
    let first_run = config::init_config()?;
    let _cfg = {
        if first_run {
            config::config::default()
        } else {
            config::read_config()?
        }
    };

    print!("┌──────┐    ");
    data::line("OS", data::os::distro());
    print!("│ ┌────┴─┐  ");
    data::line("DE", data::desktop::desktop());
    print!("└─┤  >_  │  ");
    data::line("SH", data::shell::shell());
    print!("  └──────┘  ");
    data::line("UP", data::uptime::uptime());

    Ok(())
}
