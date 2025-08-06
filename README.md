# 🌌 Galaxy Simulation

A stunning GPU-accelerated galaxy simulation featuring thousands of stars orbiting a central mass. Built with **Rust**, **WebAssembly**, and **WebGPU** for maximum performance directly in your browser.

![Galaxy Simulation Demo](https://via.placeholder.com/800x400/000011/ffffff?text=🌌+Galaxy+Simulation+Demo)

## ✨ Features

- **🚀 GPU Acceleration**: All physics calculations run on your GPU using WebGPU compute shaders
- **🦀 Rust Performance**: Written in Rust and compiled to WebAssembly for near-native speed
- **🎮 Interactive Controls**: Pan, zoom, pause, and reset with intuitive mouse and keyboard controls
- **🌈 Visual Effects**: Stars are colored based on their orbital velocity with smooth gradients
- **📱 Responsive Design**: Works on desktop and mobile devices with WebGPU support
- **☁️ Edge Deployment**: Ready for deployment on Cloudflare Workers for global distribution
- **🧵 Future Threading**: Prepared for multi-threading with `wasm-bindgen-rayon` when browsers support it

## 🎯 Live Demo

**Coming Soon**: The simulation will be deployed at `https://galaxy-sim.your-workers-domain.workers.dev`

## 🎮 Controls

| Input           | Action                           |
| --------------- | -------------------------------- |
| **Mouse Drag**  | Pan the camera around the galaxy |
| **Mouse Wheel** | Zoom in and out                  |
| **Spacebar**    | Pause/resume the simulation      |
| **R Key**       | Reset camera to default position |

## 🏗️ Architecture

### Technology Stack

- **Backend**: Rust with `wgpu` for WebGPU access
- **Frontend**: WebAssembly with minimal JavaScript glue
- **Graphics**: WGSL compute and render shaders
- **Math**: `cgmath` for linear algebra operations
- **Build**: `wasm-pack` for WebAssembly compilation
- **Deploy**: Cloudflare Workers for edge distribution

### GPU Pipeline

1. **Compute Shader** (`update.wgsl`): Updates particle positions and velocities in parallel
2. **Render Shader** (`render.wgsl`): Draws particles with velocity-based coloring
3. **Camera System**: 2D orthographic projection with pan/zoom controls

### Project Structure

```
galacto/
├── src/                    # Rust source code
│   ├── lib.rs             # Main WASM entry point
│   ├── graphics.rs        # WebGPU initialization and management
│   ├── simulation.rs      # GPU simulation logic and buffers
│   ├── camera.rs          # Camera transforms and controls
│   ├── input.rs           # Event handling and user interaction
│   ├── render.rs          # Rendering utilities
│   ├── utils.rs           # Helper functions and performance monitoring
│   └── shaders/           # WGSL compute and render shaders
│       ├── update.wgsl    # Particle physics compute shader
│       └── render.wgsl    # Particle rendering vertex/fragment shaders
├── static/                 # Web assets
│   ├── index.html         # Main application page
│   └── styles.css         # UI styling and responsive layout
├── pkg/                    # Generated WebAssembly output (build target)
├── scripts/                # Build and utility scripts
├── Cargo.toml             # Rust dependencies and configuration
├── package.json           # Node.js build scripts and dev dependencies
├── wrangler.toml          # Cloudflare Workers configuration
├── server.js              # Development server with proper MIME types
└── README.md              # This documentation
```

## 🚀 Quick Start

### Prerequisites

- **Rust**: Install from [rustup.rs](https://rustup.rs/)
- **Node.js**: Version 16+ for build scripts and dev server
- **WebGPU Browser**: Chrome 113+, Edge 113+, or Firefox with WebGPU enabled

### Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/galaxy-sim.git
cd galaxy-sim

# Set up the development environment
npm run setup

# Build the WebAssembly module
npm run build

# Start the development server
npm run dev
```

Open `http://localhost:8000` in a WebGPU-enabled browser.

## 🛠️ Development

### Build Commands

| Command              | Description                              |
| -------------------- | ---------------------------------------- |
| `npm run setup`      | Install dependencies and add WASM target |
| `npm run build`      | Build WASM module and copy assets        |
| `npm run build:wasm` | Compile Rust to WebAssembly only         |
| `npm run dev`        | Build and start development server       |
| `npm run serve`      | Start server (requires prior build)      |
| `npm run clean`      | Clean build artifacts                    |
| `npm run test`       | Run Rust tests                           |
| `npm run lint`       | Run Clippy linter                        |
| `npm run format`     | Format Rust code                         |

### Manual Build Steps

```bash
# 1. Compile Rust to WebAssembly
wasm-pack build --target web --release --out-dir pkg --out-name galaxy_sim

# 2. Copy static assets
cp static/* pkg/

# 3. Start development server
node server.js
```

### Debugging

The development server includes helpful error messages and performance monitoring:

- **WebGPU Errors**: Detailed error messages for unsupported browsers
- **Performance Stats**: FPS and frame time logging every 5 seconds
- **Console Logging**: Rust panics are displayed in browser console

## 🌐 Browser Support

### WebGPU Compatibility

| Browser         | Status   | Notes                                         |
| --------------- | -------- | --------------------------------------------- |
| **Chrome/Edge** | ✅ 113+  | WebGPU enabled by default                     |
| **Firefox**     | 🔧 110+  | Enable `dom.webgpu.enabled` in `about:config` |
| **Safari**      | ⚠️ 16.4+ | WebGPU support varies by version              |

### Enabling WebGPU

#### Chrome/Edge

WebGPU should be enabled by default. If not, visit `chrome://flags/` and enable:

- "Unsafe WebGPU" (if needed)

#### Firefox

1. Type `about:config` in the address bar
2. Search for `dom.webgpu.enabled`
3. Set to `true`
4. Restart the browser

#### Safari

WebGPU support is gradually rolling out. Check if your version supports it.

## 📊 Performance

The simulation is designed to run smoothly with thousands of particles:

- **Target**: 60 FPS with 4,096 particles
- **GPU Memory**: ~32KB for particle data
- **Compute**: Single dispatch per frame (~64 workgroups)
- **Rendering**: Point primitives with velocity-based coloring

### Performance Monitoring

The application includes built-in performance monitoring:

- FPS counter in the UI
- Frame time logging to console
- Automatic performance adjustment for slower devices

## 🧵 Future: Multi-threading

The project is prepared for WebAssembly threading support:

### Enabling Threads

1. **Update toolchain**:

   ```toml
   # rust-toolchain.toml
   [toolchain]
   channel = "nightly-2024-08-02"
   ```

2. **Enable build flags**:

   ```toml
   # .cargo/config.toml
   [target.wasm32-unknown-unknown]
   rustflags = ["-C", "target-feature=+atomics,+bulk-memory"]

   [unstable]
   build-std = ["panic_abort", "std"]
   ```

3. **Uncomment dependencies**:

   ```toml
   # Cargo.toml
   rayon = "1.8"
   wasm-bindgen-rayon = { version = "1.2", features = ["no-bundler"] }
   ```

4. **Update HTML initialization**:
   ```javascript
   import init, { initThreadPool } from "./galaxy_sim.js";
   await init();
   await initThreadPool(navigator.hardwareConcurrency);
   ```

### Browser Requirements for Threading

Threading requires Cross-Origin Isolation:

- `Cross-Origin-Opener-Policy: same-origin`
- `Cross-Origin-Embedder-Policy: require-corp`

These headers are already configured in the development server and Cloudflare Worker.

## ☁️ Deployment

### Cloudflare Workers

The project is configured for easy deployment to Cloudflare Workers:

```bash
# Deploy to production
npm run deploy

# Test worker locally
npm run dev:worker
```

### Configuration

Update `wrangler.toml`:

```toml
name = "your-galaxy-sim"
# Add your account details and routes
[[routes]]
pattern = "*/*"
zone_name = "your-domain.com"
```

### Alternative Deployment

The app is a static web application and can be deployed anywhere:

- **GitHub Pages**: Upload `pkg/` contents
- **Netlify/Vercel**: Point to `pkg/` directory
- **AWS S3**: Static website hosting
- **Any CDN**: All files are self-contained

## 🔬 Physics Model

### Gravitational Simulation

The simulation uses a simplified N-body gravitational model:

1. **Central Mass**: Fixed gravitational source at origin
2. **Particle Motion**: Euler integration with gravitational acceleration
3. **Orbital Mechanics**: Circular orbital velocities with random perturbations
4. **Boundary Conditions**: Elastic collisions with world boundaries

### Shader Implementation

**Compute Shader** (`update.wgsl`):

```wgsl
// Gravitational acceleration: a = -GM/r^3 * position_vector
let acceleration = -params.gm * inv_r3 * particle.position;

// Euler integration
particle.velocity = particle.velocity * drag + acceleration * params.dt;
particle.position = particle.position + particle.velocity * params.dt;
```

**Render Shader** (`render.wgsl`):

- Velocity-based coloring (blue → cyan → yellow → red)
- Point primitive rendering
- Camera transformation

## 🎨 Customization

### Simulation Parameters

Modify `simulation.rs`:

```rust
const NUM_PARTICLES: u32 = 4096;  // Number of stars
let params = SimulationParams {
    dt: 0.016,          // Time step (60 FPS)
    gm: 50000.0,        // Gravitational strength
    particle_count: NUM_PARTICLES,
};
```

### Visual Style

Modify `render.wgsl`:

```wgsl
// Change color scheme
let color = mix(vec3<f32>(0.2, 0.4, 1.0), vec3<f32>(1.0, 0.3, 0.0), speed_factor);
```

### Galaxy Distribution

Modify particle generation in `simulation.rs`:

```rust
// Create different galaxy shapes
let radius = rng.gen_range(50.0..400.0);  // Disk galaxy
// Or try: let radius = rng.gen_range(0.0..400.0).powf(0.5) * 400.0; // Dense center
```

## 🐛 Troubleshooting

### Common Issues

1. **"WebGPU not supported"**

   - Update your browser to the latest version
   - Enable WebGPU flags as described above
   - Try a different browser

2. **Build errors**

   ```bash
   # Ensure correct Rust target
   rustup target add wasm32-unknown-unknown

   # Clean and rebuild
   npm run clean && npm run build
   ```

3. **WASM loading errors**

   - Use the provided development server (proper MIME types)
   - Don't open `file://` URLs directly
   - Check browser console for detailed errors

4. **Performance issues**
   - Reduce `NUM_PARTICLES` in `simulation.rs`
   - Check if WebGPU is actually being used (not WebGL fallback)
   - Monitor FPS in the UI controls

### Getting Help

- **Issues**: Open a GitHub issue with browser version and error details
- **Discussions**: Use GitHub Discussions for questions and ideas
- **Contributing**: See CONTRIBUTING.md (if available)

## 🚧 Roadmap

### Planned Features

- **3D Visualization**: Perspective camera with orbit controls
- **Multiple Galaxies**: Galaxy collision simulations
- **Particle Types**: Different star types with varying properties
- **Environmental Effects**: Dark matter, dust clouds, supernovae
- **Data Export**: Save simulation states and recordings
- **VR Support**: WebXR integration for immersive experience

### Performance Improvements

- **LOD System**: Level-of-detail for distant particles
- **Culling**: Frustum and distance-based particle culling
- **Instancing**: Hardware instancing for better GPU utilization
- **Compute Optimization**: Shared memory and workgroup optimization

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- **Rust Community**: For the amazing ecosystem and tools
- **WebGPU Team**: For bringing GPU computing to the web
- **wgpu Developers**: For the excellent Rust graphics library
- **Cloudflare**: For providing edge computing infrastructure
- **Inspiration**: Based on the architecture of [rgilks/evo](https://github.com/rgilks/evo)

---

**Built with ❤️ and 🦀 by the Galaxy Sim Team**

_Watch the universe unfold in your browser!_ 🌌
