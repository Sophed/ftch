# ftch

![image](https://github.com/user-attachments/assets/0618aa0f-f70c-4513-82c1-47236b06bc24)

## Installation
- ftch is available on the [AUR](https://aur.archlinux.org/packages/ftch-bin) as `ftch-bin`
- It can also be installed through cargo with `cargo install ftch`

## Configuring
Config file can be found in `XDG_CONFIG_HOME`, usually just `~/.config/ftch/config.toml`
```toml
[display]
ascii = "stack"
seperator = ": "
modules = ["os", "desktop", "shell", "uptime"]

[colours]
primary = "\u001B[39m" # reset
accent = "\u001B[36m" # cyan
```
More info about ansi colours codes [here](https://gist.github.com/JBlond/2fea43a3049b38287e5e9cefc87b2124)

## Building From Source
```
git clone https://github.com/Sophed/ftch
cd ftch
cargo build --release
```
