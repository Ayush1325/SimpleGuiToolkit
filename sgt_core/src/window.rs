use futures::channel::oneshot;

pub trait WindowManager<T> {
    fn run(handler: Self, proxy_tx: oneshot::Sender<T>) -> !;
}

#[derive(Debug)]
pub enum CustomEvents {
    CreateNewWindow,
    CloseWindow,
}

pub struct EventLoopError;
