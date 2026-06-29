use ai_traffic_lights::app_state::{RuntimeStatusSource, SharedAppState};
use ai_traffic_lights::domain::aggregate_display_state;
use ai_traffic_lights::domain::display_state::TrafficLightDisplayState;
use ai_traffic_lights::domain::preferences::{TrafficLightAppearancePreset, UserDisplayPreference};
use ai_traffic_lights::providers::codex_app_server::PROVIDER_NAME as CODEX_APP_SERVER_PROVIDER_NAME;
use ai_traffic_lights::providers::codex_hooks::{
    self, CodexHooksInstallStatus, PROVIDER_NAME as CODEX_HOOKS_PROVIDER_NAME,
};
use ai_traffic_lights::providers::jsonl_replay::load_replay_file;
use ai_traffic_lights::providers::StatusProviderHealth;
use ai_traffic_lights::storage::preferences_store;
use std::path::PathBuf;
use tauri::menu::MenuBuilder;
use tauri::tray::TrayIconBuilder;
use tauri::{Manager, State};

const MENU_INSTALL_HOOKS: &str = "install-hooks";
const MENU_TOGGLE_LIGHT: &str = "toggle-light";
const MENU_QUIT: &str = "quit";
const MENU_APPEARANCE_CLASSIC: &str = "appearance-classic";
const MENU_APPEARANCE_MINIMAL: &str = "appearance-minimal";
const MENU_APPEARANCE_NEON: &str = "appearance-neon";
const MENU_APPEARANCE_HIGH_CONTRAST: &str = "appearance-high-contrast";

fn main() {
    if let Some(exit_code) = run_cli_diagnostic() {
        std::process::exit(exit_code);
    }

    let (display, preferences_path, status_source) = initial_display_state();
    let preferences = preferences_store::load_preferences(&preferences_path).unwrap_or_default();
    let app_state = SharedAppState::new_with_status_source(
        display,
        preferences,
        preferences_path,
        status_source,
    );

    tauri::Builder::default()
        .manage(app_state)
        .setup(|app| {
            setup_tray_menu(app)?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            current_display_state,
            current_preferences,
            set_compact_mode,
            set_appearance_preset,
            codex_hooks_status,
            install_codex_hooks
        ])
        .run(tauri::generate_context!())
        .expect("error while running AI Traffic Lights");
}

fn run_cli_diagnostic() -> Option<i32> {
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|arg| arg == "--print-hooks-status") {
        return Some(print_json(&codex_hooks::inspect_installation()));
    }
    if args.iter().any(|arg| arg == "--print-display-state") {
        return Some(print_json(&codex_hooks::current_display_state()));
    }
    None
}

fn print_json(value: &impl serde::Serialize) -> i32 {
    match serde_json::to_string_pretty(value) {
        Ok(output) => {
            println!("{output}");
            0
        }
        Err(error) => {
            eprintln!("failed to render diagnostic JSON: {error}");
            1
        }
    }
}

fn setup_tray_menu(app: &mut tauri::App) -> tauri::Result<()> {
    let appearance_menu = tauri::menu::SubmenuBuilder::new(app, "Appearance")
        .text(MENU_APPEARANCE_CLASSIC, "Classic")
        .text(MENU_APPEARANCE_MINIMAL, "Minimal")
        .text(MENU_APPEARANCE_NEON, "Neon")
        .text(MENU_APPEARANCE_HIGH_CONTRAST, "High Contrast")
        .build()?;

    let tray_menu = MenuBuilder::new(app)
        .text(MENU_INSTALL_HOOKS, "Install Hooks")
        .text(MENU_TOGGLE_LIGHT, "Show/Hide Light")
        .item(&appearance_menu)
        .separator()
        .text(MENU_QUIT, "Quit")
        .build()?;

    let mut tray = TrayIconBuilder::new()
        .menu(&tray_menu)
        .tooltip("AI Traffic Light")
        .show_menu_on_left_click(true)
        .on_menu_event(|app, event| match event.id().as_ref() {
            MENU_INSTALL_HOOKS => {
                if let Err(error) = codex_hooks::install_hooks() {
                    eprintln!("failed to install Codex hooks: {error}");
                }
            }
            MENU_TOGGLE_LIGHT => toggle_main_window(app),
            MENU_QUIT => app.exit(0),
            id => {
                if let Some(preset) = appearance_preset_from_menu_id(id) {
                    let state = app.state::<SharedAppState>();
                    if let Err(error) = state.set_appearance_preset(preset) {
                        eprintln!("failed to save appearance preset: {error}");
                    }
                }
            }
        });

    if let Some(icon) = app.default_window_icon() {
        tray = tray.icon(icon.clone());
    }

    tray.build(app)?;
    Ok(())
}

fn appearance_preset_from_menu_id(id: &str) -> Option<TrafficLightAppearancePreset> {
    match id {
        MENU_APPEARANCE_CLASSIC => Some(TrafficLightAppearancePreset::Classic),
        MENU_APPEARANCE_MINIMAL => Some(TrafficLightAppearancePreset::Minimal),
        MENU_APPEARANCE_NEON => Some(TrafficLightAppearancePreset::Neon),
        MENU_APPEARANCE_HIGH_CONTRAST => Some(TrafficLightAppearancePreset::HighContrast),
        _ => None,
    }
}

fn toggle_main_window(app: &tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        if window.is_visible().unwrap_or(true) {
            let _ = window.hide();
        } else {
            let _ = window.show();
        }
    }
}

#[tauri::command]
fn current_display_state(state: State<'_, SharedAppState>) -> TrafficLightDisplayState {
    state.current_display()
}

#[tauri::command]
fn current_preferences(state: State<'_, SharedAppState>) -> UserDisplayPreference {
    state.preferences()
}

#[tauri::command]
fn set_compact_mode(
    state: State<'_, SharedAppState>,
    enabled: bool,
) -> Result<UserDisplayPreference, String> {
    state.set_compact(enabled)
}

#[tauri::command]
fn set_appearance_preset(
    state: State<'_, SharedAppState>,
    preset: TrafficLightAppearancePreset,
) -> Result<UserDisplayPreference, String> {
    state.set_appearance_preset(preset)
}

#[tauri::command]
fn codex_hooks_status() -> CodexHooksInstallStatus {
    codex_hooks::inspect_installation()
}

#[tauri::command]
fn install_codex_hooks() -> Result<CodexHooksInstallStatus, String> {
    codex_hooks::install_hooks()
}

fn initial_display_state() -> (
    TrafficLightDisplayState,
    PathBuf,
    Option<RuntimeStatusSource>,
) {
    let preferences_path = preferences_store::default_preferences_path()
        .unwrap_or_else(|_| PathBuf::from("preferences.json"));
    let args: Vec<String> = std::env::args().collect();
    let source = args
        .windows(2)
        .find(|pair| pair[0] == "--status-source")
        .map(|pair| pair[1].clone());

    let active_source = source.as_deref().unwrap_or(CODEX_HOOKS_PROVIDER_NAME);
    let (display, runtime_source) = match active_source {
        CODEX_HOOKS_PROVIDER_NAME => (
            codex_hooks::current_display_state(),
            Some(RuntimeStatusSource::CodexHooks),
        ),
        CODEX_APP_SERVER_PROVIDER_NAME => (
            TrafficLightDisplayState::unavailable("codex_app_server_event_names_unverified", None),
            None,
        ),
        path => match load_replay_file(path) {
            Ok(signals) => (
                aggregate_display_state(&signals, &StatusProviderHealth::replaying("jsonl-replay")),
                None,
            ),
            Err(error) => (TrafficLightDisplayState::unavailable(error, None), None),
        },
    };
    (display, preferences_path, runtime_source)
}
