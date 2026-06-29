use crate::domain::display_state::TrafficLightDisplayState;
use crate::domain::preferences::{TrafficLightAppearancePreset, UserDisplayPreference};
use crate::providers::codex_hooks;
use crate::storage::preferences_store;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use tauri::State;

#[derive(Clone)]
pub struct SharedAppState {
    inner: Arc<AppStateInner>,
}

struct AppStateInner {
    display_state: RwLock<TrafficLightDisplayState>,
    preferences: RwLock<UserDisplayPreference>,
    preferences_path: PathBuf,
    status_source: Option<RuntimeStatusSource>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeStatusSource {
    CodexHooks,
}

impl SharedAppState {
    pub fn new(
        display_state: TrafficLightDisplayState,
        preferences: UserDisplayPreference,
        preferences_path: PathBuf,
    ) -> Self {
        Self {
            inner: Arc::new(AppStateInner {
                display_state: RwLock::new(display_state),
                preferences: RwLock::new(preferences),
                preferences_path,
                status_source: None,
            }),
        }
    }

    pub fn new_with_status_source(
        display_state: TrafficLightDisplayState,
        preferences: UserDisplayPreference,
        preferences_path: PathBuf,
        status_source: Option<RuntimeStatusSource>,
    ) -> Self {
        Self {
            inner: Arc::new(AppStateInner {
                display_state: RwLock::new(display_state),
                preferences: RwLock::new(preferences),
                preferences_path,
                status_source,
            }),
        }
    }

    pub fn current_display(&self) -> TrafficLightDisplayState {
        self.refresh_from_runtime_source();
        self.inner
            .display_state
            .read()
            .expect("display state lock")
            .clone()
    }

    fn refresh_from_runtime_source(&self) {
        if matches!(
            self.inner.status_source,
            Some(RuntimeStatusSource::CodexHooks)
        ) {
            self.set_display(codex_hooks::current_display_state());
        }
    }

    pub fn set_display(&self, state: TrafficLightDisplayState) {
        *self
            .inner
            .display_state
            .write()
            .expect("display state lock") = state;
    }

    pub fn preferences(&self) -> UserDisplayPreference {
        self.inner
            .preferences
            .read()
            .expect("preferences lock")
            .clone()
    }

    pub fn set_compact(&self, enabled: bool) -> Result<UserDisplayPreference, String> {
        let updated = self.preferences().with_compact_mode(enabled);
        preferences_store::save_preferences(&self.inner.preferences_path, &updated)?;
        *self.inner.preferences.write().expect("preferences lock") = updated.clone();
        Ok(updated)
    }

    pub fn set_appearance_preset(
        &self,
        preset: TrafficLightAppearancePreset,
    ) -> Result<UserDisplayPreference, String> {
        let updated = self.preferences().with_appearance_preset(preset);
        preferences_store::save_preferences(&self.inner.preferences_path, &updated)?;
        *self.inner.preferences.write().expect("preferences lock") = updated.clone();
        Ok(updated)
    }
}

#[tauri::command]
pub fn current_display_state(state: State<'_, SharedAppState>) -> TrafficLightDisplayState {
    state.current_display()
}

#[tauri::command]
pub fn current_preferences(state: State<'_, SharedAppState>) -> UserDisplayPreference {
    state.preferences()
}

#[tauri::command]
pub fn set_compact_mode(
    state: State<'_, SharedAppState>,
    enabled: bool,
) -> Result<UserDisplayPreference, String> {
    state.set_compact(enabled)
}

#[tauri::command]
pub fn set_appearance_preset(
    state: State<'_, SharedAppState>,
    preset: TrafficLightAppearancePreset,
) -> Result<UserDisplayPreference, String> {
    state.set_appearance_preset(preset)
}
