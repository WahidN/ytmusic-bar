use std::{fs, path::PathBuf, sync::Mutex};

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};

#[derive(Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Settings {
    pub show_in_notch: bool,
    pub global_shortcut: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            show_in_notch: true,
            global_shortcut: true,
        }
    }
}

fn path(app: &AppHandle) -> Option<PathBuf> {
    app.path().app_config_dir().ok().map(|dir| dir.join("settings.json"))
}

// Geen of kapot bestand is geen fout: dan gelden de standaardwaarden.
pub fn load(app: &AppHandle) -> Settings {
    path(app)
        .and_then(|path| fs::read_to_string(path).ok())
        .and_then(|json| serde_json::from_str(&json).ok())
        .unwrap_or_default()
}

fn save(app: &AppHandle, settings: &Settings) {
    let Some(path) = path(app) else {
        return;
    };
    if let Some(dir) = path.parent() {
        let _ = fs::create_dir_all(dir);
    }
    if let Ok(json) = serde_json::to_string_pretty(settings) {
        let _ = fs::write(path, json);
    }
}

pub fn update(app: &AppHandle, change: impl FnOnce(&mut Settings)) {
    let state = app.state::<Mutex<Settings>>();
    let mut settings = state.lock().unwrap();
    change(&mut settings);
    save(app, &settings);
}
