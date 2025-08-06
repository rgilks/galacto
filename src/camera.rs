use cgmath::{ortho, perspective, Deg, EuclideanSpace, Matrix4, Point3, SquareMatrix, Vector3};

pub struct Camera {
    pub position: Vector3<f32>,
    pub scale: f32,
    pub aspect_ratio: f32,
    pub is_3d: bool,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            position: Vector3::new(0.0, 0.0, 800.0),
            scale: 1.0,
            aspect_ratio: 1.0,
            is_3d: false,
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

    pub fn zoom(&mut self, delta: f32) {
        // Exponential zoom for smooth feel
        let zoom_factor = 1.0 + delta * 0.001;
        self.scale *= zoom_factor;

        // Clamp zoom levels
        self.scale = self.scale.clamp(0.01, 10.0);
    }

    pub fn reset(&mut self) {
        self.position = Vector3::new(0.0, 0.0, 800.0);
        self.scale = 1.0;
    }

    pub fn build_view_projection_matrix(&self) -> Matrix4<f32> {
        if self.is_3d {
            // 3D perspective projection
            let view = Matrix4::look_at_rh(
                Point3::from_vec(self.position),
                Point3::new(0.0, 0.0, 0.0),
                Vector3::unit_y(),
            );

            let proj = perspective(Deg(45.0), self.aspect_ratio, 0.1, 2000.0);

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

    pub fn world_to_screen(
        &self,
        world_pos: Vector3<f32>,
        screen_width: f32,
        screen_height: f32,
    ) -> Vector3<f32> {
        let mvp = self.build_view_projection_matrix();
        let clip_pos = mvp * world_pos.extend(1.0);

        // Perspective divide
        let ndc = clip_pos.truncate() / clip_pos.w;

        // Convert to screen coordinates
        Vector3::new(
            (ndc.x + 1.0) * 0.5 * screen_width,
            (1.0 - ndc.y) * 0.5 * screen_height,
            ndc.z,
        )
    }

    pub fn screen_to_world(
        &self,
        screen_pos: Vector3<f32>,
        screen_width: f32,
        screen_height: f32,
    ) -> Vector3<f32> {
        // Convert screen to NDC
        let ndc_x = (screen_pos.x / screen_width) * 2.0 - 1.0;
        let ndc_y = 1.0 - (screen_pos.y / screen_height) * 2.0;

        let mvp = self.build_view_projection_matrix();
        let inv_mvp = mvp.invert().unwrap_or(Matrix4::from_scale(1.0));

        let world_pos = inv_mvp * cgmath::Vector4::new(ndc_x, ndc_y, 0.0, 1.0);
        world_pos.truncate() / world_pos.w
    }
}
