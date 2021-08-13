use futures::channel::oneshot;
use sgt_core::window::{CustomEvents, WindowManager};
use std::collections::HashMap;
pub use winit::event_loop::EventLoopProxy;
use winit::{
    event_loop::{EventLoop, EventLoopWindowTarget},
    window::{Window, WindowId},
};

#[derive(Debug, Default)]
pub struct Winit {
    windows: HashMap<WindowId, Window>,
}

impl Winit {
    fn create_event_loop() -> EventLoop<CustomEvents> {
        EventLoop::with_user_event()
    }

    fn create_window<T>(
        &mut self,
        event_loop: &EventLoopWindowTarget<T>,
    ) -> Result<(), winit::error::OsError> {
        use winit::window::WindowBuilder;

        let window = WindowBuilder::new().build(event_loop)?;
        self.windows.insert(window.id(), window);
        Ok(())
    }

    fn remove_window(&mut self, id: &WindowId) {
        self.windows.remove(id);
    }

    fn window_present(&self) -> bool {
        !self.windows.is_empty()
    }

    fn redraw_window(&self, id: &WindowId) {
        if let Some(x) = self.windows.get(id) {
            x.request_redraw();
        }
    }
}

impl WindowManager<EventLoopProxy<CustomEvents>> for Winit {
    fn run(mut handler: Self, proxy_tx: oneshot::Sender<EventLoopProxy<CustomEvents>>) -> ! {
        use winit::{
            event::ElementState, event::Event, event::KeyboardInput, event::VirtualKeyCode,
            event::WindowEvent, event_loop::ControlFlow,
        };

        let event_loop = Self::create_event_loop();
        let proxy = event_loop.create_proxy();
        proxy_tx.send(proxy);

        handler.create_window(&event_loop).unwrap();

        event_loop.run(move |event, event_loop, control_flow| {
            *control_flow = ControlFlow::Wait;
            match event {
                Event::WindowEvent { event, window_id } => match event {
                    WindowEvent::CloseRequested => {
                        handler.remove_window(&window_id);
                        if !handler.window_present() {
                            *control_flow = ControlFlow::Exit;
                        }
                    }
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Space),
                                ..
                            },
                        ..
                    } => {
                        handler.create_window(&event_loop).unwrap();
                    }
                    _ => (),
                },
                Event::UserEvent(x) => {
                    println!("Here: {:#?}", x);
                }
                Event::RedrawRequested(id) => {
                    handler.redraw_window(&id);
                }
                _ => (),
            }
        })
    }
}
