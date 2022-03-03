#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use self::windows::*;
#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::*;

mod error;
pub use error::*;

/// Information about a window just getting into foreground
pub struct WindowInformation {
    pub title: String,
    pub executable: String,
}
