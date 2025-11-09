use glam::{Mat4, Vec3};

// note to self: use the wgpu convention for z-depth [0, 1], rather than [-1, 1]
// as per OpenGL

pub struct Camera {
    pub eye: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
}

impl Camera {
    pub fn build_view_projection_matrix(&self) -> Mat4 {
        let view = glam::f32::Mat4::look_at_rh(self.eye, self.target, self.up);
        Mat4::orthographic_rh(left, right, bottom, top, self.znear, self.zfar)
    }
}
