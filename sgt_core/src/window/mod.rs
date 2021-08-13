#[cfg(feature = "sgt_winit")]
mod winit;

use futures::channel::oneshot;

pub trait WindowManager {
    type EventSender;

    fn run(handler: Self, proxy_tx: oneshot::Sender<Self::EventSender>) -> !;
}

#[derive(Debug)]
pub enum CustomEvents {
    CreateNewWindow,
    CloseWindow,
}

pub struct EventLoopError;
