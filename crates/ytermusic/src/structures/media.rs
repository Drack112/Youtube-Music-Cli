#[cfg(not(target_os = "macos"))]
use flume::Sender;

#[cfg(not(target_os = "macos"))]
use crate::term::ManagerMessage;

#[cfg(not(target_os = "macos"))]
pub fn run_window_handler(_updater: &Sender<ManagerMessage>) -> Option<()> {
    loop {
        use crate::shutdown::is_shutdown_sent;

        if is_shutdown_sent() {
            use std::process::exit;

            use log::info;

            info!("event loop closed");
            exit(0);
        }
    }
}
