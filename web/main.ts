import init, { run } from "../pkg/fractal_madness.js";

// Theme Configuration
const THEMES = [
  { id: 'gallery', name: 'Gallery' },
  { id: 'cyberpunk', name: 'Cyberpunk' },
  { id: 'retro', name: 'Retro Terminal' },
  { id: 'deep-space', name: 'Deep Space' },
  { id: 'sketchbook', name: 'Sketchbook' },
  { id: 'neo-brutalism', name: 'Neo-Brutalism' },
  { id: 'glass', name: 'Glassmorphism' },
  { id: '8bit', name: '8-Bit OS' },
  { id: 'vaporwave', name: 'Vaporwave' },
  { id: 'industrial', name: 'Industrial HUD' }
];

class ThemeManager {
  private currentThemeIndex: number = 2; // Default to Retro Terminal (index 2)
  private btn: HTMLButtonElement | null;
  private label: HTMLElement | null;

  constructor() {
    this.btn = document.querySelector('#theme-toggle');
    this.label = document.querySelector('.current-theme-name');
    
    // Always default to 8-Bit OS (index 7) on load
    this.currentThemeIndex = 7;

    this.applyTheme();
    // Click listener disabled - title box only
  }

  private initListeners() {
    this.btn?.addEventListener('click', () => this.cycleTheme());
  }

  private cycleTheme() {
    this.currentThemeIndex = (this.currentThemeIndex + 1) % THEMES.length;
    this.applyTheme();
  }

  private applyTheme() {
    const theme = THEMES[this.currentThemeIndex];
    
    // Apply to HTML tag
    document.documentElement.setAttribute('data-theme', theme.id);
    
    // Update button label
    if (this.label) {
      this.label.textContent = theme.name;
    }
  }
}

async function main() {
  // Initialize Theme System
  new ThemeManager();

  const errorBanner = document.getElementById("error-banner");

  // Check for WebGPU support
  if (!navigator.gpu) {
    console.error("WebGPU is not supported in this browser");
    errorBanner?.classList.remove("hidden");
    return;
  }

  try {
    // Initialize the WASM module
    await init();
    // Run the application
    await run();
  } catch (error) {
    console.error("Failed to initialize fractal visualizer:", error);
    if (errorBanner) {
      errorBanner.querySelector("h2")!.textContent = "Initialization Error";
      errorBanner.querySelector("p")!.textContent =
        `Failed to initialize the fractal visualizer: ${error}`;
      errorBanner.classList.remove("hidden");
    }
  }
}

main();
