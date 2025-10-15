use std::env;

pub fn shell() -> String {
    match env::var("SHELL") {
        Ok(s) => basename(&s).to_string(),
        Err(_) => "Unknown".to_string(),
    }
}

fn basename(s: &String) -> &str {
    let parts: Vec<&str> = s.split("/").collect();
    parts.last().copied().unwrap()
}
