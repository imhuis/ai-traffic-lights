export type Display = "green" | "red" | "yellow" | "unavailable";
export type AppearancePreset = "classic" | "minimal" | "neon" | "high_contrast";

export interface TrafficLightDisplayState {
  version: "1";
  display: Display;
  label: string;
  accessibility_label: string;
  mapped_meaning: string;
  last_update?: string;
  is_stale: boolean;
  active_session_count: number;
  waiting_session_count: number;
  working_session_count: number;
  unavailable_reason?: string;
}

export interface UserDisplayPreference {
  window_position?: {
    screen: string;
    x: number;
    y: number;
  };
  window_size?: {
    width: number;
    height: number;
  };
  always_on_top: boolean;
  compact_mode: boolean;
  appearance_preset: AppearancePreset;
  show_in_tray: boolean;
  launch_at_login: boolean;
  updated_at: string;
}

export interface TrafficLightRenderOptions {
  appearanceMenuOpen: boolean;
  onToggleAppearanceMenu: () => void;
  onSelectAppearancePreset: (preset: AppearancePreset) => void | Promise<void>;
}

const lights: Display[] = ["red", "yellow", "green"];
export const appearancePresets: AppearancePreset[] = [
  "classic",
  "minimal",
  "neon",
  "high_contrast",
];

const appearancePresetLabels: Record<AppearancePreset, string> = {
  classic: "Classic",
  minimal: "Minimal",
  neon: "Neon",
  high_contrast: "High contrast",
};

export function renderTrafficLight(
  root: HTMLElement,
  state: TrafficLightDisplayState,
  preferences: UserDisplayPreference,
  options: TrafficLightRenderOptions,
): void {
  root.innerHTML = "";

  const panel = document.createElement("section");
  const panelClasses = [
    "traffic-panel",
    `state-${state.display}`,
    `appearance-${preferences.appearance_preset}`,
  ];
  if (preferences.compact_mode) {
    panelClasses.push("mode-compact");
  }
  panel.className = panelClasses.join(" ");
  panel.setAttribute("aria-label", state.accessibility_label);
  panel.title = state.label;

  panel.appendChild(createAppearanceControls(preferences, options));

  const stack = document.createElement("div");
  stack.className = "traffic-stack";

  for (const light of lights) {
    const bulb = document.createElement("div");
    bulb.className = `traffic-bulb bulb-${light}`;
    bulb.setAttribute("aria-hidden", "true");
    if (state.display === light) {
      bulb.classList.add("active");
    }
    stack.appendChild(bulb);
  }

  if (state.display === "unavailable") {
    stack.classList.add("unavailable");
  }

  panel.append(stack);
  root.appendChild(panel);
}

function createAppearanceControls(
  preferences: UserDisplayPreference,
  options: TrafficLightRenderOptions,
): DocumentFragment {
  const fragment = document.createDocumentFragment();

  const toggle = document.createElement("button");
  toggle.type = "button";
  toggle.className = "appearance-toggle";
  toggle.title = "Appearance";
  toggle.setAttribute("aria-label", "Appearance settings");
  toggle.setAttribute("aria-expanded", String(options.appearanceMenuOpen));
  toggle.addEventListener("click", () => options.onToggleAppearanceMenu());

  const toggleIcon = document.createElement("span");
  toggleIcon.className = "appearance-toggle-icon";
  toggleIcon.setAttribute("aria-hidden", "true");
  toggle.appendChild(toggleIcon);
  fragment.appendChild(toggle);

  const menu = document.createElement("div");
  menu.className = "appearance-menu";
  menu.setAttribute("role", "menu");
  menu.setAttribute("aria-label", "Traffic light appearance");
  menu.hidden = !options.appearanceMenuOpen;

  for (const preset of appearancePresets) {
    const choice = document.createElement("button");
    choice.type = "button";
    choice.className = `appearance-choice choice-${preset}`;
    choice.title = appearancePresetLabels[preset];
    choice.setAttribute("role", "menuitemradio");
    choice.setAttribute("aria-label", appearancePresetLabels[preset]);
    choice.setAttribute("aria-checked", String(preferences.appearance_preset === preset));
    if (preferences.appearance_preset === preset) {
      choice.classList.add("selected");
    }
    choice.addEventListener("click", () => {
      void options.onSelectAppearancePreset(preset);
    });
    choice.appendChild(createAppearanceSwatch(preset));
    menu.appendChild(choice);
  }

  fragment.appendChild(menu);
  return fragment;
}

function createAppearanceSwatch(preset: AppearancePreset): HTMLElement {
  const swatch = document.createElement("span");
  swatch.className = `appearance-swatch swatch-${preset}`;
  swatch.setAttribute("aria-hidden", "true");

  for (const light of lights) {
    const dot = document.createElement("span");
    dot.className = `appearance-swatch-dot dot-${light}`;
    swatch.appendChild(dot);
  }

  return swatch;
}
