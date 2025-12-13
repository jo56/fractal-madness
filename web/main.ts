import init, { run } from "../pkg/fractal_madness.js";

// Theme Configuration
const THEMES = [
  { id: 'gallery', name: 'Gallery' },
  { id: 'cyberpunk', name: 'Cyberpunk' },
  { id: 'retro', name: 'Retro Terminal' },
  { id: 'deep-space', name: 'Deep Space' },
  { id: 'sketchbook', name: 'Sketchbook' }
];

class ThemeManager {
  private currentThemeIndex: number = 0;
  private btn: HTMLButtonElement | null;
  private label: HTMLElement | null;

  constructor() {
    this.btn = document.querySelector('#theme-toggle');
    this.label = document.querySelector('.current-theme-name');
    
    // Load saved theme or default to 0
    const savedTheme = localStorage.getItem('fractal-theme');
    if (savedTheme) {
      const index = THEMES.findIndex(t => t.id === savedTheme);
      if (index !== -1) this.currentThemeIndex = index;
    }

    this.applyTheme();
    this.initListeners();
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

    // Save persistence
    localStorage.setItem('fractal-theme', theme.id);
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
