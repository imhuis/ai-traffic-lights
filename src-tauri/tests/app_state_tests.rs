use ai_traffic_lights::app_state::SharedAppState;
use ai_traffic_lights::domain::{
    DisplayColor, TrafficLightAppearancePreset, TrafficLightDisplayState, UserDisplayPreference,
};
use chrono::Utc;
use tempfile::tempdir;

#[test]
fn stores_and_returns_current_display_state() {
    let dir = tempdir().expect("tempdir");
    let app = SharedAppState::new(
        TrafficLightDisplayState::red(Utc::now(), 1, 1),
        UserDisplayPreference::default(),
        dir.path().join("preferences.json"),
    );

    let display = app.current_display();
    assert_eq!(display.display, DisplayColor::Red);
    assert_eq!(display.working_session_count, 1);
}

#[test]
fn stores_yellow_display_fields_and_counts() {
    let dir = tempdir().expect("tempdir");
    let app = SharedAppState::new(
        TrafficLightDisplayState::yellow(Utc::now(), 2, 1, 1),
        UserDisplayPreference::default(),
        dir.path().join("preferences.json"),
    );

    let display = app.current_display();
    assert_eq!(display.display, DisplayColor::Yellow);
    assert_eq!(display.waiting_session_count, 1);
    assert!(display.accessibility_label.contains("confirmation"));
}

#[test]
fn saves_compact_mode_preference() {
    let dir = tempdir().expect("tempdir");
    let path = dir.path().join("preferences.json");
    let app = SharedAppState::new(
        TrafficLightDisplayState::unavailable("no_status_source", None),
        UserDisplayPreference::default(),
        path.clone(),
    );

    let updated = app.set_compact(true).expect("save compact");
    assert!(updated.compact_mode);
    let saved = std::fs::read_to_string(path).expect("saved preferences");
    assert!(saved.contains("\"compact_mode\": true"));
}

#[test]
fn saves_appearance_preset_preference() {
    let dir = tempdir().expect("tempdir");
    let path = dir.path().join("preferences.json");
    let app = SharedAppState::new(
        TrafficLightDisplayState::unavailable("no_status_source", None),
        UserDisplayPreference::default(),
        path.clone(),
    );

    let updated = app
        .set_appearance_preset(TrafficLightAppearancePreset::HighContrast)
        .expect("save appearance preset");

    assert_eq!(
        updated.appearance_preset,
        TrafficLightAppearancePreset::HighContrast
    );
    let saved = std::fs::read_to_string(path).expect("saved preferences");
    assert!(saved.contains("\"appearance_preset\": \"high_contrast\""));
}
