use crate::domain::preferences::UserDisplayPreference;
use directories::ProjectDirs;
use std::fs;
use std::path::{Path, PathBuf};

pub fn default_preferences_path() -> Result<PathBuf, String> {
    let dirs = ProjectDirs::from("local", "ai-traffic-lights", "AI Traffic Lights")
        .ok_or_else(|| "could not locate platform config directory".to_string())?;
    Ok(dirs.config_dir().join("preferences.json"))
}

pub fn load_preferences(path: impl AsRef<Path>) -> Result<UserDisplayPreference, String> {
    let path = path.as_ref();
    if !path.exists() {
        return Ok(UserDisplayPreference::default());
    }
    let content =
        fs::read_to_string(path).map_err(|error| format!("failed to read preferences: {error}"))?;
    let prefs = serde_json::from_str(&content)
        .map_err(|error| format!("failed to parse preferences: {error}"))?;
    Ok(prefs)
}

pub fn save_preferences(
    path: impl AsRef<Path>,
    preferences: &UserDisplayPreference,
) -> Result<(), String> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| format!("failed to create preferences directory: {error}"))?;
    }
    let content = serde_json::to_string_pretty(preferences)
        .map_err(|error| format!("failed to serialize preferences: {error}"))?;
    fs::write(path, content).map_err(|error| format!("failed to write preferences: {error}"))
}
