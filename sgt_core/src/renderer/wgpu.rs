use super::Renderer;
use sgt_wgpu::{RenderSize, WGPU};

impl Renderer for WGPU {
    fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.size = RenderSize::new(width, height);
            self.swap_chain_desc.width = width;
            self.swap_chain_desc.height = height;
            self.swap_chain = self
                .device
                .create_swap_chain(&self.surface, &self.swap_chain_desc);
        }
    }
}
