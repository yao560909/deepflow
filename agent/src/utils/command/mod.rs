#[cfg(any(target_os = "linux", target_os = "android"))]
mod linux;
#[cfg(any(target_os = "linux", target_os = "android"))]
pub use linux::*;


#[cfg(any(target_os = "macos"))]
mod linux;

#[cfg(any(target_os = "macos"))]
pub use linux::*;