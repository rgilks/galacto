// Compute shader for updating particle positions and velocities
struct Particle {
    position: vec3<f32>,
    velocity: vec3<f32>,
}

struct Params {
    dt: f32,
    gm: f32,        // Gravitational parameter (G * central_mass)
    particle_count: u32,
    _padding: u32,
}

@group(0) @binding(0) var<storage, read_write> particles: array<Particle>;
@group(0) @binding(1) var<uniform> params: Params;

@compute @workgroup_size(64)
fn update_particles(@builtin(global_invocation_id) gid: vec3<u32>) {
    let index = gid.x;
    if (index >= params.particle_count) {
        return;
    }
    
    var particle = particles[index];
    
    // Debug: log first few particles to see if they're moving
    if (index < 5u) {
        // We can't use console.log in WGSL, but we can make particles more visible
        // by giving them a special color or position
    }
    
    // Calculate distance from center (0, 0, 0)
    let r2 = dot(particle.position, particle.position) + 1e-6; // Add small epsilon to avoid division by zero
    let r = sqrt(r2);
    let inv_r = 1.0 / r;
    let inv_r3 = inv_r * inv_r * inv_r;
    
    // Gravitational acceleration towards center: a = -GM/r^3 * position_vector
    let acceleration = -params.gm * inv_r3 * particle.position;
    
    // Add small amount of drag to prevent runaway velocities
    let drag = 0.999;
    
    // Euler integration
    particle.velocity = particle.velocity * drag + acceleration * params.dt;
    particle.position = particle.position + particle.velocity * params.dt;
    
    // Boundary conditions - bounce off edges in 3D
    let boundary = 600.0;
    if (abs(particle.position.x) > boundary) {
        particle.position.x = sign(particle.position.x) * boundary;
        particle.velocity.x = -particle.velocity.x * 0.8;
    }
    if (abs(particle.position.y) > boundary) {
        particle.position.y = sign(particle.position.y) * boundary;
        particle.velocity.y = -particle.velocity.y * 0.8;
    }
    if (abs(particle.position.z) > boundary) {
        particle.position.z = sign(particle.position.z) * boundary;
        particle.velocity.z = -particle.velocity.z * 0.8;
    }
    
    particles[index] = particle;
}