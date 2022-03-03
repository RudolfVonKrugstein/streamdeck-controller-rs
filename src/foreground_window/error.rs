#[cfg(target_os = "linux")]
use x11rb::errors::{ConnectError, ConnectionError, ReplyError};

#[cfg(target_os = "linux")]
#[derive(Debug)]
pub enum X11Error {
    ConnectError(ConnectError),
    ConnectionError(ConnectionError),
    ReplyError(ReplyError),
}

#[cfg(target_os = "windows")]
#[derive(Debug)]
pub enum Win32Error {
    UnspecificError,
    CoreError(windows::core::Error),
}

#[derive(Debug)]
pub enum Error {
    #[cfg(target_os = "linux")]
    WMError(X11Error),
    #[cfg(target_os = "windows")]
    WMError(Win32Error),
    AlreadyStarted,
}
