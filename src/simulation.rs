use crate::utils::console_log;
use bytemuck::{Pod, Zeroable};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use wgpu::util::DeviceExt;

const NUM_PARTICLES: u32 = 4096;
const WORKGROUP_SIZE: u32 = 64;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Particle {
    pub position: [f32; 2],
    pub velocity: [f32; 2],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct SimulationParams {
    pub dt: f32,
    pub gm: f32, // Gravitational parameter (G * central_mass)
    pub particle_count: u32,
    pub _padding: u32,
}

pub struct Simulation {
    pub particle_buffer: wgpu::Buffer,
    pub params_buffer: wgpu::Buffer,
    pub compute_pipeline: wgpu::ComputePipeline,
    pub render_pipeline: wgpu::RenderPipeline,
    pub compute_bind_group: wgpu::BindGroup,
    pub render_bind_group: wgpu::BindGroup,
    pub camera_buffer: wgpu::Buffer,
    params: SimulationParams,
}

impl Simulation {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue) -> Result<Self, wasm_bindgen::JsValue> {
        console_log!("Creating simulation...");

        // Generate initial particle data
        let particles = Self::generate_initial_particles();

        // Create particle buffer
        let particle_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Particle Buffer"),
            contents: bytemuck::cast_slice(&particles),
            usage: wgpu::BufferUsages::STORAGE
                | wgpu::BufferUsages::VERTEX
                | wgpu::BufferUsages::COPY_DST,
        });

        // Create simulation parameters
        let params = SimulationParams {
            dt: 0.016,   // ~60fps
            gm: 50000.0, // Gravitational parameter
            particle_count: NUM_PARTICLES,
            _padding: 0,
        };

        let params_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Params Buffer"),
            contents: bytemuck::cast_slice(&[params]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // Create camera buffer
        let camera_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Camera Buffer"),
            size: 64, // 4x4 matrix = 16 * 4 bytes
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Load and create compute shader
        let compute_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Compute Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/update.wgsl").into()),
        });

        // Load and create render shader
        let render_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Render Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/render.wgsl").into()),
        });

        // Create compute bind group layout
        let compute_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Compute Bind Group Layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: false },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                ],
            });

        // Create render bind group layout
        let render_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Render Bind Group Layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: true },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                ],
            });

        // Create compute pipeline
        let compute_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Compute Pipeline Layout"),
                bind_group_layouts: &[&compute_bind_group_layout],
                push_constant_ranges: &[],
            });

        let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("Compute Pipeline"),
            layout: Some(&compute_pipeline_layout),
            module: &compute_shader,
            entry_point: "update_particles",
        });

        // Create render pipeline
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&render_bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &render_shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &render_shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Bgra8UnormSrgb,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::PointList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        // Create bind groups
        let compute_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Compute Bind Group"),
            layout: &compute_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: particle_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: params_buffer.as_entire_binding(),
                },
            ],
        });

        let render_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Render Bind Group"),
            layout: &render_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: camera_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: particle_buffer.as_entire_binding(),
                },
            ],
        });

        console_log!("Simulation created with {} particles", NUM_PARTICLES);

        Ok(Self {
            particle_buffer,
            params_buffer,
            compute_pipeline,
            render_pipeline,
            compute_bind_group,
            render_bind_group,
            camera_buffer,
            params,
        })
    }

    fn generate_initial_particles() -> Vec<Particle> {
        let mut rng = StdRng::seed_from_u64(42);
        let mut particles = Vec::with_capacity(NUM_PARTICLES as usize);

        for _ in 0..NUM_PARTICLES {
            // Create a disk galaxy distribution
            let radius = rng.gen_range(50.0..400.0);
            let angle = rng.gen_range(0.0..2.0 * std::f32::consts::PI);

            let x = radius * angle.cos();
            let y = radius * angle.sin();

            // Circular orbital velocity (simplified)
            let orbital_speed = (50000.0 / radius).sqrt() * 0.8; // Reduced for stability
            let vx = -orbital_speed * angle.sin() + rng.gen_range(-10.0..10.0);
            let vy = orbital_speed * angle.cos() + rng.gen_range(-10.0..10.0);

            particles.push(Particle {
                position: [x, y],
                velocity: [vx, vy],
            });
        }

        particles
    }

    pub fn update(&mut self, queue: &wgpu::Queue, dt: f32) {
        self.params.dt = dt.min(0.033); // Cap at ~30fps for stability
        queue.write_buffer(&self.params_buffer, 0, bytemuck::cast_slice(&[self.params]));
    }

    pub fn compute_pass(&self, encoder: &mut wgpu::CommandEncoder) {
        let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("Compute Pass"),
            timestamp_writes: None,
        });

        compute_pass.set_pipeline(&self.compute_pipeline);
        compute_pass.set_bind_group(0, &self.compute_bind_group, &[]);
        compute_pass.dispatch_workgroups(
            (NUM_PARTICLES + WORKGROUP_SIZE - 1) / WORKGROUP_SIZE,
            1,
            1,
        );
    }

    pub fn render_pass<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.render_bind_group, &[]);
        render_pass.draw(0..NUM_PARTICLES, 0..1);
    }

    pub fn update_camera(&self, queue: &wgpu::Queue, camera: &crate::camera::Camera) {
        let matrix = camera.build_view_projection_matrix();
        let matrix_array: &[f32; 16] = matrix.as_ref();
        queue.write_buffer(&self.camera_buffer, 0, bytemuck::cast_slice(matrix_array));
    }
}
