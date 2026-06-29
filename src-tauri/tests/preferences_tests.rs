use ai_traffic_lights::domain::{
    TrafficLightAppearancePreset, UserDisplayPreference, WindowPosition,
};
use ai_traffic_lights::storage::preferences_store::{load_preferences, save_preferences};
use tempfile::tempdir;

#[test]
fn persists_display_preferences_only() {
    let dir = tempdir().expect("tempdir");
    let path = dir.path().join("preferences.json");
    let preference = UserDisplayPreference {
        window_position: Some(WindowPosition {
            screen: "main".to_string(),
            x: 9000,
            y: -100,
        }),
        compact_mode: true,
        appearance_preset: TrafficLightAppearancePreset::Neon,
        ..UserDisplayPreference::default()
    };

    save_preferences(&path, &preference).expect("save preferences");
    let loaded = load_preferences(&path).expect("load preferences");

    assert!(loaded.compact_mode);
    assert_eq!(loaded.appearance_preset, TrafficLightAppearancePreset::Neon);
    assert!(loaded.window_position.is_some());
    let serialized = std::fs::read_to_string(path).expect("preference text");
    assert!(serialized.contains("\"appearance_preset\": \"neon\""));
    assert!(!serialized.contains("session_id"));
    assert!(!serialized.contains("command"));
}

#[test]
fn loads_missing_appearance_preset_as_classic() {
    let dir = tempdir().expect("tempdir");
    let path = dir.path().join("preferences.json");
    std::fs::write(
        &path,
        r#"{
  "window_size": {
    "width": 92,
    "height": 220
  },
  "always_on_top": true,
  "compact_mode": false,
  "show_in_tray": true,
  "launch_at_login": false,
  "updated_at": "2026-06-29T00:00:00Z"
}"#,
    )
    .expect("write old preferences");

    let loaded = load_preferences(&path).expect("load old preferences");

    assert_eq!(
        loaded.appearance_preset,
        TrafficLightAppearancePreset::Classic
    );
}

#[test]
fn clamps_saved_window_position_to_visible_area() {
    let mut preference = UserDisplayPreference {
        window_position: Some(WindowPosition {
            screen: "main".to_string(),
            x: 9000,
            y: -50,
        }),
        ..UserDisplayPreference::default()
    };

    preference.clamp_to_visible(800, 600);
    let position = preference.window_position.expect("position");
    assert_eq!(position.x, 760);
    assert_eq!(position.y, 0);
}
