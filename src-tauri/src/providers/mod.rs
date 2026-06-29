pub mod codex_app_server;
pub mod codex_hooks;
pub mod jsonl_replay;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderStatus {
    Connected,
    Disconnected,
    Degraded,
    Replaying,
    Invalid,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatusProviderHealth {
    pub provider: String,
    pub status: ProviderStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_heartbeat: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_error_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_error_message: Option<String>,
}

impl StatusProviderHealth {
    pub fn connected(provider: impl Into<String>) -> Self {
        Self {
            provider: provider.into(),
            status: ProviderStatus::Connected,
            last_heartbeat: Some(Utc::now()),
            last_error_code: None,
            last_error_message: None,
        }
    }

    pub fn replaying(provider: impl Into<String>) -> Self {
        Self {
            provider: provider.into(),
            status: ProviderStatus::Replaying,
            last_heartbeat: Some(Utc::now()),
            last_error_code: None,
            last_error_message: None,
        }
    }

    pub fn disconnected(provider: impl Into<String>, reason: impl Into<String>) -> Self {
        Self {
            provider: provider.into(),
            status: ProviderStatus::Disconnected,
            last_heartbeat: None,
            last_error_code: Some(reason.into()),
            last_error_message: None,
        }
    }
}

pub trait StatusProvider {
    fn provider_name(&self) -> &str;
}
