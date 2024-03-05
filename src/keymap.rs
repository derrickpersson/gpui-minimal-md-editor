use serde::Deserialize;
use serde_json;
use std::fs;

#[derive(Deserialize)]
struct Keybindings {
    // Define your fields here
}

#[cfg(target_os = "macos")]
pub const DEFAULT_KEYMAP_PATH: &str = "keymaps/default-macos.json";

pub fn default_keymap() -> Keybindings {
    let json_contents = fs::read_to_string(DEFAULT_KEYMAP_PATH)
        .expect("Failed to read JSON file");
    serde_json::from_str(&json_contents)
        .expect("Failed to deserialize JSON")
}