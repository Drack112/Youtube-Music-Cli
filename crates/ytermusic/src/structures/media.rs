#[cfg(not(target_os = "macos"))]
use flume::Sender;
#[cfg(not(target_os = "windows"))]
use souvlaki::MediaControls;

#[cfg(not(target_os = "macos"))]
use crate::term::ManagerMessage;

#[cfg(not(target_os = "windows"))]
fn get_handle(updater: &Sender<ManagerMessage>) -> Option<MediaControls> {
    use crate::errors::handle_error_option;
    use souvlaki::PlatformConfig;
    handle_error_option(
        updater,
        "Can't create media controls",
        MediaControls::new(PlatformConfig {
            dbus_name: "ytermusic",
            display_name: "YTerMusic",
            hwnd: None,
        })
        .map_err(|e| format!("{e:?}")),
    )
}
#[cfg(not(target_os = "macos"))]
pub fn run_window_handler(_updater: &Sender<ManagerMessage>) -> Option<()> {
    use crate::shutdown::is_shutdown_sent;

    loop {
        if is_shutdown_sent() {
            use std::process::exit;

            use log::info;

            info!("event loop closed");
            exit(0);
        }
    }
}
