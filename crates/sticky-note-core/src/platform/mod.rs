//! Platform-specific window abstractions for desktop layering and stealth modes.

#[cfg(windows)]
pub mod windows;
#[cfg(windows)]
pub use windows::*;

#[cfg(not(windows))]
pub mod stubs;
#[cfg(not(windows))]
pub use stubs::*;
