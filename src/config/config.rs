use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Config {
    display: DisplayConfig,
    colours: ColoursConfig,
}

#[derive(Deserialize, Serialize)]
struct DisplayConfig {
    ascii: String,
    seperator: String,
    modules: Vec<String>,
}

#[derive(Deserialize, Serialize)]
struct ColoursConfig {
    primary: String,
    accent: String,
}

pub fn default() -> Config {
    Config {
        display: DisplayConfig {
            ascii: "stack".to_string(),
            seperator: ": ".to_string(),
            modules: vec![
                "os".to_string(),
                "desktop".to_string(),
                "shell".to_string(),
                "uptime".to_string(),
            ],
        },
        colours: ColoursConfig {
            primary: "\x1B[39m".to_string(),
            accent: "\x1B[36m".to_string(),
        },
    }
}
