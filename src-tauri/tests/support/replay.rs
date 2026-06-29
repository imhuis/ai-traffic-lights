use ai_traffic_lights::domain::{CodexStatusSignal, RawStatusEvent};
use std::path::{Path, PathBuf};

pub fn fixture_path(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures/status-sequences")
        .join(name)
}

pub fn signal_from_json(value: &str) -> CodexStatusSignal {
    let raw: RawStatusEvent = serde_json::from_str(value).expect("valid raw status event");
    let received_at = raw.observed_at;
    raw.into_signal(received_at).expect("valid signal")
}
