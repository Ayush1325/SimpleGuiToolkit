use crate::window::WindowManager;

#[derive(Debug)]
pub struct Application<T>
where
    T: WindowManager,
{
    window_manager: T,
}

impl<T> Application<T>
where
    T: WindowManager,
{
    pub fn new(window_manager: T) -> Self {
        Self { window_manager }
    }

    pub fn run_common(self) -> ! {
        use futures::channel::oneshot;

        let (tx, rx) = oneshot::channel();

        T::run(self.window_manager, tx)
    }
}
