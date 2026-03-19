#[derive(Clone, Copy)]
pub struct DrawParams {
    pub transform: glam::Mat4,
    pub color: glam::U8Vec4,
    pub texture_window: glam::Vec4,
}

impl Default for DrawParams {
    fn default() -> Self {
        Self {
            transform: glam::Mat4::IDENTITY,
            color: glam::U8Vec4::new(255, 255, 255, 255),
            texture_window: glam::Vec4::new(0.0, 0.0, 1.0, 1.0),
        }
    }
}
