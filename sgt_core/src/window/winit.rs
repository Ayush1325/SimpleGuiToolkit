use super::{CustomEvents, WindowManager};
use crate::application::Application;
use futures::channel::oneshot;
use sgt_winit::{winit, Winit};
use winit::event_loop::{EventLoop, EventLoopProxy};

impl WindowManager for Winit {
    type EventSender = EventLoopProxy<CustomEvents>;

    fn run(mut handler: Self, proxy_tx: oneshot::Sender<Self::EventSender>) -> ! {
        use winit::{
            event::ElementState, event::Event, event::KeyboardInput, event::VirtualKeyCode,
            event::WindowEvent, event_loop::ControlFlow,
        };

        let event_loop = EventLoop::with_user_event();
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

impl Application<Winit> {
    pub fn run() -> ! {
        let winit_handler = Winit::default();
        Self::run_common(winit_handler)
    }
}
