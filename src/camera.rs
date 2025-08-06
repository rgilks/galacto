use cgmath::{ortho, perspective, Deg, EuclideanSpace, Matrix4, Point3, Vector3};

pub struct Camera {
    pub position: Vector3<f32>,
    pub scale: f32,
    pub aspect_ratio: f32,
    pub is_3d: bool,
    pub rotation_x: f32,
    pub rotation_y: f32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            position: Vector3::new(0.0, 0.0, 800.0),
            scale: 1.0,
            aspect_ratio: 1.0,
            is_3d: true, // Enable 3D by default
            rotation_x: 0.0,
            rotation_y: 0.0,
        }
    }

    pub fn set_aspect_ratio(&mut self, aspect_ratio: f32) {
        self.aspect_ratio = aspect_ratio;
    }

    pub fn pan(&mut self, delta_x: f32, delta_y: f32) {
        // Scale the pan based on current zoom level
        let pan_scale = 1.0 / self.scale;
        self.position.x -= delta_x * pan_scale;
        self.position.y += delta_y * pan_scale; // Flip Y for screen coordinates
    }

    pub fn rotate(&mut self, delta_x: f32, delta_y: f32) {
        self.rotation_y += delta_x;
        self.rotation_x += delta_y;

        // Clamp vertical rotation to prevent flipping
        self.rotation_x = self.rotation_x.clamp(-1.5, 1.5);
    }

    pub fn zoom(&mut self, delta: f32) {
        // Exponential zoom for smooth feel
        let zoom_factor = 1.0 + delta * 0.001;
        self.scale *= zoom_factor;

        // Clamp zoom levels - prevent zooming out too far
        self.scale = self.scale.clamp(0.1, 20.0);
    }

    pub fn reset(&mut self) {
        self.position = Vector3::new(0.0, 0.0, 800.0);
        self.scale = 1.0;
        self.rotation_x = 0.0;
        self.rotation_y = 0.0;
    }

    pub fn build_view_projection_matrix(&self) -> Matrix4<f32> {
        if self.is_3d {
            // 3D perspective projection with rotation
            let distance = 800.0 / self.scale;

            // Calculate rotated position
            let rot_x = cgmath::Matrix3::from_angle_x(cgmath::Rad(self.rotation_x));
            let rot_y = cgmath::Matrix3::from_angle_y(cgmath::Rad(self.rotation_y));
            let rotation = rot_y * rot_x;

            let rotated_position = rotation * Vector3::new(0.0, 0.0, distance);
            let camera_pos = Point3::from_vec(rotated_position);

            let view =
                Matrix4::look_at_rh(camera_pos, Point3::new(0.0, 0.0, 0.0), Vector3::unit_y());

            // Increase far plane to prevent particles disappearing when zoomed out
            let proj = perspective(Deg(45.0), self.aspect_ratio, 0.1, 5000.0);

            proj * view
        } else {
            // 2D orthographic projection
            let view_width = 1000.0 / self.scale;
            let view_height = view_width / self.aspect_ratio;

            let left = self.position.x - view_width / 2.0;
            let right = self.position.x + view_width / 2.0;
            let bottom = self.position.y - view_height / 2.0;
            let top = self.position.y + view_height / 2.0;

            ortho(left, right, bottom, top, -1000.0, 1000.0)
        }
    }
}
