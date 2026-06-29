use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrafficLightAppearancePreset {
    #[default]
    Classic,
    Minimal,
    Neon,
    HighContrast,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WindowPosition {
    pub screen: String,
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WindowSize {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserDisplayPreference {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub window_position: Option<WindowPosition>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub window_size: Option<WindowSize>,
    pub always_on_top: bool,
    pub compact_mode: bool,
    #[serde(default)]
    pub appearance_preset: TrafficLightAppearancePreset,
    pub show_in_tray: bool,
    pub launch_at_login: bool,
    pub updated_at: DateTime<Utc>,
}

impl Default for UserDisplayPreference {
    fn default() -> Self {
        Self {
            window_position: None,
            window_size: Some(WindowSize {
                width: 220,
                height: 320,
            }),
            always_on_top: true,
            compact_mode: false,
            appearance_preset: TrafficLightAppearancePreset::default(),
            show_in_tray: true,
            launch_at_login: false,
            updated_at: Utc::now(),
        }
    }
}

impl UserDisplayPreference {
    pub fn with_compact_mode(mut self, enabled: bool) -> Self {
        self.compact_mode = enabled;
        self.updated_at = Utc::now();
        self
    }

    pub fn with_appearance_preset(mut self, preset: TrafficLightAppearancePreset) -> Self {
        self.appearance_preset = preset;
        self.updated_at = Utc::now();
        self
    }

    pub fn clamp_to_visible(&mut self, max_width: i32, max_height: i32) {
        if let Some(position) = &mut self.window_position {
            position.x = position.x.clamp(0, max_width.saturating_sub(40));
            position.y = position.y.clamp(0, max_height.saturating_sub(40));
        }
    }
}
