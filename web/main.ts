import init, { run } from "../pkg/fractal_madness.js";

async function main() {
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
