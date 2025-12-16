# Fractal Madness

A GPU-accelerated fractal visualizer built with Rust and WebGPU, compiled to WebAssembly for browser deployment.

## Features

- **9 Fractal Types**:
  - *Classic Escape-Time*: Mandelbrot, Tricorn, Celtic, Burning Ship
  - *Julia Variants*: Julia, Buffalo Julia, Celtic Julia
  - *Advanced*: Newton (root-finding), Phoenix (memory-based)
- **26 Color Schemes**: Classic, Fire, Ocean, Rainbow, Neon, Plasma, and more
- **Interactive Controls**: Pan with mouse drag, zoom with scroll wheel
- **Real-time Parameters**: Adjust iterations, power, escape radius
- **Julia Set Explorer**: Adjust the complex constant c in real-time
- **58+ Location Presets**: Curated locations across all fractal types
- **Performance Warnings**: Dynamic warnings for computationally expensive settings

## Tech Stack

- **Rust** - Core logic and fractal computation
- **wgpu** - WebGPU/WebGL2 graphics abstraction
- **egui** - Immediate mode GUI
- **wasm-pack** - Rust to WebAssembly compilation
- **Vite** - Frontend build tool

## Requirements

- [Rust](https://rustup.rs/) (with `wasm32-unknown-unknown` target)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- [Node.js](https://nodejs.org/) (v18+)

## Quick Start

```bash
# Install Rust WASM target
rustup target add wasm32-unknown-unknown

# Install wasm-pack
cargo install wasm-pack

# Install Node dependencies
npm install

# Development (builds WASM and starts dev server)
npm run dev

# Production build
npm run build
```

Open http://localhost:5173 in a modern browser.

## Project Structure

```
fractal-madness/
├── src/                    # Rust source
│   ├── lib.rs              # WASM entry point
│   ├── renderer.rs         # GPU rendering pipeline
│   ├── webgpu.rs           # WebGPU initialization
│   ├── ui.rs               # egui control panel
│   ├── input.rs            # Mouse input handling
│   ├── color.rs            # Color scheme definitions
│   └── fractal/            # Fractal implementations
│       ├── mandelbrot.rs
│       ├── julia.rs
│       ├── burning_ship.rs
│       ├── tricorn.rs
│       ├── celtic.rs
│       ├── buffalo.rs
│       ├── newton.rs
│       └── phoenix.rs
├── shaders/
│   └── fractal.wgsl        # WGSL fragment shader
├── web/                    # Frontend
│   ├── index.html
│   └── main.ts
├── pkg/                    # WASM build output (generated)
└── dist/                   # Production build (generated)
```

## Browser Compatibility

Uses WebGL2 backend for broad compatibility. Tested on:
- Chrome 90+
- Firefox 89+
- Edge 90+
- Safari 15+

## Controls

| Action | Input |
|--------|-------|
| Pan | Click and drag |
| Zoom | Mouse wheel |
| Reset | "Reset View" button |

## Performance Notes

- **Newton fractals** are computationally expensive (~3.5x cost vs standard fractals)
- **Phoenix fractals** have moderate overhead (~1.5x cost)
- The UI displays warnings when iteration counts exceed recommended thresholds

## License

MIT
