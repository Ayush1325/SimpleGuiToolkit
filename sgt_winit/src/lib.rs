use std::collections::HashMap;
pub use winit;
use winit::{
    event_loop::EventLoopWindowTarget,
    window::{Window, WindowId},
};

#[derive(Debug, Default)]
pub struct Winit {
    windows: HashMap<WindowId, Window>,
}

impl Winit {
    pub fn create_window<T>(
        &mut self,
        event_loop: &EventLoopWindowTarget<T>,
    ) -> Result<(), winit::error::OsError> {
        use winit::window::WindowBuilder;

        let window = WindowBuilder::new().build(event_loop)?;
        self.windows.insert(window.id(), window);
        Ok(())
    }

    pub fn remove_window(&mut self, id: &WindowId) {
        self.windows.remove(id);
    }

    pub fn window_present(&self) -> bool {
        !self.windows.is_empty()
    }

    pub fn redraw_window(&self, id: &WindowId) {
        if let Some(x) = self.windows.get(id) {
            x.request_redraw();
        }
    }
}
