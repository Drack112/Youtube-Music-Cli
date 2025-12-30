use std::{
    sync::atomic::{AtomicBool, Ordering},
    task::Poll,
};

use log::info;

static SHUTDOWN_SENT: AtomicBool = AtomicBool::new(false);

pub fn is_shutdown_sent() -> bool {
    SHUTDOWN_SENT.load(Ordering::Relaxed)
}

#[derive(Clone)]
pub struct ShutdownSignal;

impl Future for ShutdownSignal {
    type Output = ();

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        if SHUTDOWN_SENT.load(Ordering::Relaxed) {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}

pub fn shutdown() {
    SHUTDOWN_SENT.store(true, Ordering::Relaxed);
    info!("Shutdown signal sent, waiting for shutdown");
}
