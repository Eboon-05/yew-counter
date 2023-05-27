use web_sys::window;

use super::Count;

// For avoiding typos
const DARK_THEME: &str = "dark_theme";
const COUNTS: &str = "counts";

pub fn save_counts(counts: &Vec<Count>) {
    let window = window().unwrap();
    let local_storage = window.local_storage().unwrap().unwrap();

    local_storage.set_item(COUNTS, serde_json::to_string(counts).unwrap().as_str()).unwrap();
}

pub fn get_dark_theme() -> bool {
    let window = window().unwrap();

    let local_storage = window.local_storage().unwrap().unwrap();

    let ls_theme = local_storage.get_item(DARK_THEME).unwrap();

    let mut dark_theme = true;

    if ls_theme.is_some() {
        let parsed = serde_json::from_str::<bool>(&ls_theme.unwrap());

        if parsed.is_ok() {
            dark_theme = parsed.unwrap();
        }
    } else {
        local_storage.set_item(DARK_THEME, dark_theme.to_string().as_str()).unwrap();
    }

    return dark_theme
}

pub fn get_counts() -> Vec<Count> {
    let window = window().unwrap();

    let local_storage = window.local_storage().unwrap().unwrap();

    let ls_counts = local_storage.get_item(COUNTS).unwrap();

    let mut counts = Vec::from([Count {
        name: "Test count".to_string(),
        value: 0,
        tags: Vec::from(["test".to_string(), "work".to_string()]),
    }]);

    if ls_counts.is_some() {
        let parsed = serde_json::from_str::<Vec<Count>>(&ls_counts.unwrap());

        if parsed.is_ok() {
            counts = parsed.unwrap();
        }
    } else {
        local_storage.set_item(COUNTS, serde_json::to_string(&counts).unwrap().as_str()).unwrap();
    }

    return counts
}