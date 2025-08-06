// Vertex and fragment shaders for rendering particles as points

struct Particle {
    position: vec2<f32>,
    velocity: vec2<f32>,
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
    
    // Transform position to clip space
    let world_position = vec4<f32>(particle.position, 0.0, 1.0);
    let clip_position = camera.transform * world_position;
    
    // Color based on velocity magnitude (speed coloring)
    let velocity_magnitude = length(particle.velocity);
    let speed_factor = min(velocity_magnitude / 200.0, 1.0);
    
    // Create color gradient: blue (slow) -> cyan -> yellow -> red (fast)
    var color: vec3<f32>;
    if (speed_factor < 0.5) {
        // Blue to cyan
        let t = speed_factor * 2.0;
        color = mix(vec3<f32>(0.2, 0.4, 1.0), vec3<f32>(0.0, 1.0, 1.0), t);
    } else {
        // Cyan to yellow to red
        let t = (speed_factor - 0.5) * 2.0;
        color = mix(vec3<f32>(0.0, 1.0, 1.0), vec3<f32>(1.0, 1.0, 0.0), t);
        if (t > 0.7) {
            let t2 = (t - 0.7) / 0.3;
            color = mix(color, vec3<f32>(1.0, 0.3, 0.0), t2);
        }
    }
    
    var out: VertexOutput;
    out.clip_position = clip_position;
    out.color = color;
    out.velocity_magnitude = velocity_magnitude;
    
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Simple point rendering with velocity-based glow
    let glow_factor = min(in.velocity_magnitude / 100.0, 1.0);
    let final_color = in.color + vec3<f32>(glow_factor * 0.2);
    
    return vec4<f32>(final_color, 0.9);
}