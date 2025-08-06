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
    
    // Normalize velocity magnitude for color mapping (adjust range as needed)
    let normalized_speed = min(velocity_magnitude / 200.0, 1.0);
    
    // Red for fast particles, blue for slow particles
    let red_component = normalized_speed * 2.0; // Boost red intensity
    let blue_component = 1.0 - normalized_speed;
    let green_component = 0.1; // Reduce green for more contrast

    let color = vec3<f32>(red_component, green_component, blue_component);

    var out: VertexOutput;
    out.clip_position = clip_position;
    out.color = color;
    out.velocity_magnitude = velocity_magnitude;

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Enhanced point rendering with velocity-based glow and brightness
    let normalized_speed = min(in.velocity_magnitude / 200.0, 1.0);
    
    // Brightness increases with speed - much more dramatic
    let brightness_boost = 3.0 + normalized_speed * 8.0; // 3x to 11x brightness
    let final_color = in.color * brightness_boost;
    
    // Add a bright core that's also velocity-dependent - much more red
    let core_glow = vec3<f32>(0.3, 0.3, 0.3) + normalized_speed * vec3<f32>(1.5, 0.0, 0.0);
    let final_color_with_core = final_color + core_glow;
    
    // Add depth-based alpha for better 3D effect
    let alpha = mix(0.8, 1.0, in.depth);

    return vec4<f32>(final_color_with_core, alpha);
}