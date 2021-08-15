use futures::channel::oneshot;
use sgt_core::application;
use std::thread;

fn main() {
    let application = application::Application::default();
    application.run()
}
