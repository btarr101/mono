use std::hash::{Hash, Hasher};

use crate::{handle::Handle, mesh::Mesh, texture::Texture};

pub struct WgpuInstanceKey {
    pub texture: Handle<'static, Texture>,
    pub mesh: Handle<'static, Mesh>,
}

impl PartialEq for WgpuInstanceKey {
    fn eq(&self, other: &Self) -> bool {
        self.texture.maybe_read().id == other.texture.maybe_read().id && self.mesh.maybe_read().id == other.mesh.maybe_read().id
    }
}

impl Eq for WgpuInstanceKey {}

impl Hash for WgpuInstanceKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.texture.maybe_read().id.hash(state);
        self.mesh.maybe_read().id.hash(state);
    }
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::NoUninit, bytemuck::Zeroable, PartialEq, Default)]
pub struct WgpuInstance {
    pub transform: [[f32; 4]; 4],
    pub color: [u8; 4],
    pub uv_window: [f32; 4],
}
