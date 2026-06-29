pub mod aggregation;
pub mod display_state;
pub mod preferences;
pub mod status_signal;

pub use aggregation::{aggregate_display_state, aggregate_from_signals};
pub use display_state::{DisplayColor, TrafficLightDisplayState};
pub use preferences::{
    TrafficLightAppearancePreset, UserDisplayPreference, WindowPosition, WindowSize,
};
pub use status_signal::{CodexState, CodexStatusSignal, Freshness, RawStatusEvent};
