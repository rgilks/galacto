# ⚫ Black Hole Accretion Disk Simulation

A stunning GPU-accelerated black hole accretion disk simulation featuring thousands of particles orbiting a central singularity. Built with **Rust**, **WebAssembly**, and **WebGPU** for maximum performance directly in your browser.

![Black Hole Simulation Demo](screenshot.png)

## ✨ Features

- **🚀 GPU Acceleration**: All physics calculations run on your GPU using WebGPU compute shaders
- **🦀 Rust Performance**: Written in Rust and compiled to WebAssembly for near-native speed
- **🎮 Interactive 3D Controls**: Pan, zoom, rotate, pause, and reset with intuitive mouse and keyboard controls
- **🌈 Visual Effects**: Particles are colored based on their orbital velocity with smooth gradients and depth-based effects
- **📱 Responsive Design**: Works on desktop and mobile devices with WebGPU support
- **☁️ Edge Deployment**: Ready for deployment on Cloudflare Workers for global distribution
- **🧵 Future Threading**: Prepared for multi-threading with `wasm-bindgen-rayon` when browsers support it

## 🎯 Live Demo

**Live Demo**: The simulation is deployed at `http://galacto.tre.systems/`

## 🎮 Controls

| Input                | Action                               |
| -------------------- | ------------------------------------ |
| **Left Mouse Drag**  | Rotate the camera around the center  |
| **Right Mouse Drag** | Pan the camera around the black hole |
| **Mouse Wheel**      | Zoom in and out                      |
| **Spacebar**         | Pause/resume the simulation          |
| **R Key**            | Reset camera to default position     |

## 🏗️ Architecture

### Technology Stack

- **Backend**: Rust with `wgpu` for WebGPU access
- **Frontend**: WebAssembly with minimal JavaScript glue
- **Graphics**: WGSL compute and render shaders with depth testing
- **Math**: `cgmath` for linear algebra operations
- **Build**: `wasm-pack` for WebAssembly compilation
- **Deploy**: Cloudflare Workers for edge distribution

### GPU Pipeline

1. **Compute Shader** (`update.wgsl`): Updates particle positions and velocities in 3D space
2. **Render Shader** (`render.wgsl`): Draws particles with velocity-based coloring and depth effects
3. **Camera System**: 3D perspective projection with pan/zoom/rotate controls

### Project Structure

```
galacto/
├── src/                    # Rust source code
│   ├── lib.rs             # Main WASM entry point
│   ├── graphics.rs        # WebGPU initialization and depth texture management
│   ├── simulation.rs      # GPU simulation logic and 3D particle buffers
│   ├── camera.rs          # 3D camera transforms and rotation controls
│   ├── input.rs           # Event handling and 3D user interaction
│   ├── render.rs          # Rendering utilities
│   ├── utils.rs           # Helper functions and performance monitoring
│   └── shaders/           # WGSL compute and render shaders
│       ├── update.wgsl    # 3D particle physics compute shader
│       └── render.wgsl    # 3D particle rendering with depth effects
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
git clone https://github.com/yourusername/black-hole-sim.git
cd black-hole-sim

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

- **Target**: 60 FPS with 131,072 particles
- **GPU Memory**: ~3MB for particle data
- **Compute**: Single dispatch per frame (~2,048 workgroups)
- **Rendering**: Point primitives with velocity-based coloring

### Performance Monitoring

The application includes built-in performance monitoring:

- FPS logging to console every 60 frames
- Frame time logging to console
- Performance monitoring in browser console

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

## 🔬 Physics Model

### 3D Gravitational Simulation

The simulation uses a simplified 3D N-body gravitational model:

1. **Central Singularity**: Fixed gravitational source at origin (0, 0, 0) representing a black hole
2. **3D Particle Motion**: Euler integration with 3D gravitational acceleration
3. **Orbital Mechanics**: Circular orbital velocities in the xy-plane with z-axis thickness
4. **Boundary Conditions**: Elastic collisions with 3D world boundaries
5. **Depth Effects**: Particles have varying z-coordinates for realistic accretion disk thickness

### Shader Implementation

**Compute Shader** (`update.wgsl`):

```wgsl
// 3D gravitational acceleration: a = -GM/r^3 * position_vector
let acceleration = -params.gm * inv_r3 * particle.position;

// 3D Euler integration with velocity clamping
particle.velocity = particle.velocity * drag + acceleration * params.dt;
let max_velocity = 140.0;
let current_speed = length(particle.velocity);
if current_speed > max_velocity {
    particle.velocity = normalize(particle.velocity) * max_velocity;
}
particle.position = particle.position + particle.velocity * params.dt;

// 3D boundary conditions with energy loss
let boundary = 600.0;
if abs(particle.position.x) > boundary {
    particle.position.x = sign(particle.position.x) * boundary;
    particle.velocity.x = -particle.velocity.x * 0.1;
}
```

**Render Shader** (`render.wgsl`):

- 3D perspective transformation with depth testing
- Velocity-based coloring (blue → cyan → yellow → red)
- Depth-based alpha blending for 3D depth perception
- Point primitive rendering with enhanced visibility

### 3D Camera System

The camera now supports full 3D perspective projection:

- **Perspective Projection**: 45° field of view with depth testing
- **Orbit Controls**: Left-click and drag to rotate around the center
- **Pan Controls**: Right-click and drag to move the camera
- **Zoom Controls**: Mouse wheel to adjust distance
- **Depth Texture**: Proper depth testing for 3D rendering

## 🎨 Customization

### Simulation Parameters

Modify `simulation.rs`:

```rust
const NUM_PARTICLES: u32 = 131072;  // Number of particles
let params = SimulationParams {
    dt: 0.016,          // Time step (60 FPS)
    gm: 40000.0,        // Gravitational strength
    particle_count: NUM_PARTICLES,
};
```

### Visual Style

Modify `render.wgsl`:

```wgsl
// Change color scheme
let color = mix(vec3<f32>(0.2, 0.4, 1.0), vec3<f32>(1.0, 0.3, 0.0), speed_factor);

// Adjust depth-based effects
let depth_color = mix(vec3<f32>(0.2, 0.2, 0.5), vec3<f32>(1.0, 1.0, 1.0), depth_factor);
```

### 3D Accretion Disk Distribution

Modify particle generation in `simulation.rs`:

```rust
// Create 3D accretion disk with thickness
let x = 10.0;  // Fixed distance from center
let y = rng.gen_range(-150.0..150.0);  // Spread along y-axis
let z = 100.0;  // Fixed z-coordinate

// Calculate perpendicular velocity (tangential to radius)
let vx = 150.0;  // Fixed x-velocity
```

### Camera Controls

Customize camera behavior in `camera.rs`:

```rust
// Adjust rotation sensitivity
camera.rotate(delta_x * 0.01, delta_y * 0.01);

// Change perspective field of view
let proj = perspective(Deg(45.0), self.aspect_ratio, 0.1, 2000.0);
```
