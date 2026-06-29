use super::display_state::TrafficLightDisplayState;
use super::status_signal::{CodexState, CodexStatusSignal};
use crate::providers::{ProviderStatus, StatusProviderHealth};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

pub fn aggregate_display_state(
    signals: &[CodexStatusSignal],
    health: &StatusProviderHealth,
) -> TrafficLightDisplayState {
    if !matches!(
        health.status,
        ProviderStatus::Connected | ProviderStatus::Replaying
    ) {
        return TrafficLightDisplayState::unavailable(
            health
                .last_error_code
                .clone()
                .unwrap_or_else(|| "provider_untrusted".to_string()),
            latest_update(signals),
        );
    }
    aggregate_from_signals(signals)
}

pub fn aggregate_from_signals(signals: &[CodexStatusSignal]) -> TrafficLightDisplayState {
    let mut latest: HashMap<(&str, &str), &CodexStatusSignal> = HashMap::new();

    for signal in signals {
        let key = (signal.source.as_str(), signal.session_id.as_str());
        match latest.get(&key) {
            Some(existing) if existing.sequence >= signal.sequence => {}
            _ => {
                latest.insert(key, signal);
            }
        }
    }

    let fresh: Vec<&CodexStatusSignal> = latest
        .values()
        .copied()
        .filter(|signal| signal.is_fresh() && signal.state != CodexState::Unavailable)
        .collect();

    if fresh.is_empty() {
        return TrafficLightDisplayState::unavailable(
            "no_trusted_sessions",
            latest_update(signals),
        );
    }

    let active = fresh.len();
    let waiting = fresh
        .iter()
        .filter(|signal| signal.state == CodexState::WaitingForConfirmation)
        .count();
    let working = fresh
        .iter()
        .filter(|signal| signal.state == CodexState::Working)
        .count();
    let last_update = fresh
        .iter()
        .map(|signal| signal.observed_at)
        .max()
        .unwrap_or_else(Utc::now);

    if waiting > 0 {
        TrafficLightDisplayState::yellow(last_update, active, waiting, working)
    } else if working > 0 {
        TrafficLightDisplayState::red(last_update, active, working)
    } else if fresh.iter().all(|signal| signal.state == CodexState::Idle) {
        TrafficLightDisplayState::green(last_update, active)
    } else {
        TrafficLightDisplayState::unavailable("untrusted_state_mix", Some(last_update))
    }
}

fn latest_update(signals: &[CodexStatusSignal]) -> Option<DateTime<Utc>> {
    signals.iter().map(|signal| signal.observed_at).max()
}
