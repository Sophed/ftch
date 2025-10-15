mod data;

fn main() {
    print!("┌──────┐    ");
    data::line("OS", data::distro::distro());
    print!("│ ┌────┴─┐  ");
    data::line("DE", data::desktop::desktop());
    print!("└─┤  >_  │  ");
    data::line("SH", data::shell::shell());
    print!("  └──────┘  ");
    data::line("UP", data::uptime::uptime());
}
