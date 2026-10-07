//! sticky-note-core
//!
//! Core library for Sticky Note containing data models, atomic persistence,
//! and local single-instance IPC coordination.

pub mod formatting;
pub mod ipc;
pub mod models;
pub mod platform;
pub mod storage;

pub use formatting::*;
pub use ipc::*;
pub use models::*;
pub use platform::*;
pub use storage::*;
