#[cfg(feature = "sgt_wgpu")]
pub mod wgpu;

pub trait Renderer {
    fn resize(&mut self, width: u32, height: u32);
}
