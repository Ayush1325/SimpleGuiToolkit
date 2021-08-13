use futures::channel::oneshot;
use sgt_winit::{CustomEvents, EventLoopSender, WindowManager};
use std::thread;

fn main() {
    let w = sgt_winit::Winit::default();
    let (tx, rx) = oneshot::channel::<sgt_winit::EventLoopProxy<CustomEvents>>();
    thread::spawn(move || {
        futures::executor::block_on(async {
            if let Ok(x) = rx.await {
                x.send(CustomEvents::CreateNewWindow);
            }
        })
    });

    sgt_winit::Winit::run(w, tx);
}
