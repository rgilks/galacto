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
}

@group(0) @binding(0) var<uniform> camera: Camera;
@group(0) @binding(1) var<storage, read> particles: array<Particle>;

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    let particle = particles[vertex_index];

    let world_position = vec4<f32>(particle.position, 1.0);
    let clip_position = camera.transform * world_position;

    let velocity_magnitude = length(particle.velocity);
    let normalized_speed = min(velocity_magnitude / 200.0, 1.0);
    
    // Color: blue (slow) -> red (fast)
    let color = vec3<f32>(
        normalized_speed * 2.0,
        0.1,
        1.0 - normalized_speed
    );

    var out: VertexOutput;
    out.clip_position = clip_position;
    out.color = color;
    out.velocity_magnitude = velocity_magnitude;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let normalized_speed = min(in.velocity_magnitude / 200.0, 1.0);
    
    // Brightness increases with speed
    let brightness = 3.0 + normalized_speed * 8.0;
    let final_color = in.color * brightness;
    
    // Add velocity-dependent glow
    let glow = vec3<f32>(0.3, 0.3, 0.3) + normalized_speed * vec3<f32>(1.5, 0.0, 0.0);

    return vec4<f32>(final_color + glow, 0.9);
}