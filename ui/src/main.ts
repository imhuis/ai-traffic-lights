import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import {
  renderTrafficLight,
  type AppearancePreset,
  type TrafficLightDisplayState,
  type UserDisplayPreference,
} from "./traffic-light";
import "../styles/traffic-light.css";

const rootElement = document.querySelector<HTMLElement>("#app");

if (!rootElement) {
  throw new Error("Missing #app root");
}

const root: HTMLElement = rootElement;
let displayState: TrafficLightDisplayState | undefined;
let preferences: UserDisplayPreference | undefined;
let appearanceMenuOpen = false;
let refreshInFlight = false;

async function refreshDisplay(): Promise<void> {
  if (refreshInFlight) {
    return;
  }

  refreshInFlight = true;
  try {
    const [state, nextPreferences] = await Promise.all([
      invoke<TrafficLightDisplayState>("current_display_state"),
      invoke<UserDisplayPreference>("current_preferences"),
    ]);
    displayState = state;
    preferences = normalizePreferences(nextPreferences);
    render();
  } finally {
    refreshInFlight = false;
  }
}

function render(): void {
  if (!displayState || !preferences) {
    return;
  }

  renderTrafficLight(root, displayState, preferences, {
    appearanceMenuOpen,
    onToggleAppearanceMenu: () => {
      appearanceMenuOpen = !appearanceMenuOpen;
      render();
    },
    onSelectAppearancePreset: selectAppearancePreset,
  });
}

async function selectAppearancePreset(preset: AppearancePreset): Promise<void> {
  preferences = normalizePreferences(
    await invoke<UserDisplayPreference>("set_appearance_preset", { preset }),
  );
  appearanceMenuOpen = false;
  render();
}

function normalizePreferences(nextPreferences: UserDisplayPreference): UserDisplayPreference {
  return {
    ...nextPreferences,
    appearance_preset: nextPreferences.appearance_preset ?? "classic",
  };
}

void refreshDisplay();
window.setInterval(() => void refreshDisplay(), 1000);
window.addEventListener("keydown", (event) => {
  if (event.key === "Escape" && appearanceMenuOpen) {
    appearanceMenuOpen = false;
    render();
  }
});
root.addEventListener("mousedown", (event) => {
  if (event.button !== 0 || isInteractiveDragTarget(event.target)) {
    return;
  }

  void getCurrentWindow().startDragging();
});

function isInteractiveDragTarget(target: EventTarget | null): boolean {
  if (!(target instanceof Element)) {
    return false;
  }

  return Boolean(target.closest(".appearance-toggle, .appearance-menu"));
}
