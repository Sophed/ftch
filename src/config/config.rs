use serde::{
    Deserialize,
    Serialize,
};

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub display: DisplayConfig,
    pub colours: ColoursConfig,
}

#[derive(Deserialize, Serialize)]
pub struct DisplayConfig {
    pub ascii: String,
    pub seperator: String,
    pub modules: Vec<String>,
    pub lowercase: bool,
}

#[derive(Deserialize, Serialize)]
pub struct ColoursConfig {
    pub primary: String,
    pub accent: String,
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
            lowercase: false,
        },
        colours: ColoursConfig {
            primary: "\x1B[39m".to_string(),
            accent: "\x1B[36m".to_string(),
        },
    }
}
