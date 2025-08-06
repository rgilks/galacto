// Vertex and fragment shaders for rendering particles as points

struct Particle {
    position: vec3<f32>,
    velocity: vec3<f32>,
}

struct Camera {
    transform: mat4x4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
    @location(1) velocity_magnitude: f32,
    @location(2) depth: f32,
}

@group(0) @binding(0) var<uniform> camera: Camera;
@group(0) @binding(1) var<storage, read> particles: array<Particle>;

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    let particle = particles[vertex_index];
    
    // Transform position to clip space
    let world_position = vec4<f32>(particle.position, 1.0);
    let clip_position = camera.transform * world_position;
    
    // Color based on velocity magnitude (speed coloring)
    let velocity_magnitude = length(particle.velocity);
    
    // Boost brightness significantly
    let color = vec3<f32>(1.0, 1.0, 1.0);
    
    var out: VertexOutput;
    out.clip_position = clip_position;
    out.color = color;
    out.velocity_magnitude = velocity_magnitude;
    
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Enhanced point rendering with velocity-based glow and brightness
    let glow_factor = min(in.velocity_magnitude / 100.0, 1.0);
    let brightness_boost = 3.0; // Much more brightness boost
    let final_color = (in.color + vec3<f32>(glow_factor * 1.0)) * brightness_boost;
    
    // Add a bright white core for much better visibility
    let core_glow = vec3<f32>(0.8, 0.8, 0.8);
    let final_color_with_core = final_color + core_glow;
    
    // Add depth-based alpha for better 3D effect
    let alpha = mix(0.8, 1.0, in.depth);
    
    return vec4<f32>(final_color_with_core, alpha);
}