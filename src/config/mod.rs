use std::env::{
    self,
    VarError,
};
use std::error::Error;
use std::fs;

pub mod config;

pub fn config_dir() -> Result<String, VarError> {
    if cfg!(target_os = "windows") {
        let root = env::var("USERPROFILE")?;
        Ok(format!("{}\\.ftch", root))
    } else {
        let root = env::var("XDG_CONFIG_HOME")
            .or_else(|_| env::var("HOME").map(|h| format!("{}/.config", h)))?;
        Ok(format!("{}/ftch", root))
    }
}

pub fn init_config() -> Result<bool, Box<dyn Error>> {
    let dir = config_dir()?;
    if !fs::exists(&dir)? {
        fs::create_dir(&dir)?;
        let file = format!("{}/config.toml", dir);
        let cfg = config::default();
        let contents = toml::to_string(&cfg)?;
        fs::write(&file, contents)?;
        println!("[!] created config at: {file}");
        return Ok(true);
    }
    Ok(false)
}

pub fn read_config() -> Result<config::Config, Box<dyn Error>> {
    let file = format!("{}/config.toml", config_dir()?);
    let contents = fs::read_to_string(file)?;
    let cfg: config::Config = toml::from_str(contents.as_str())?;
    Ok(cfg)
}
